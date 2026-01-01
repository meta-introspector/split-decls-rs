/* FP:check_unused.rs-0001 */ //
/* FP:check_unused.rs-0002 */ // Unused import checking
/* FP:check_unused.rs-0003 */ //
/* FP:check_unused.rs-0004 */ // Although this is mostly a lint pass, it lives in here because it depends on
/* FP:check_unused.rs-0005 */ // resolve data structures and because it finalises the privacy information for
/* FP:check_unused.rs-0006 */ // `use` items.
/* FP:check_unused.rs-0007 */ //
/* FP:check_unused.rs-0008 */ // Unused trait imports can't be checked until the method resolution. We save
/* FP:check_unused.rs-0009 */ // candidates here, and do the actual check in rustc_hir_analysis/check_unused.rs.
/* FP:check_unused.rs-0010 */ //
/* FP:check_unused.rs-0011 */ // Checking for unused imports is split into three steps:
/* FP:check_unused.rs-0012 */ //
/* FP:check_unused.rs-0013 */ //  - `UnusedImportCheckVisitor` walks the AST to find all the unused imports
/* FP:check_unused.rs-0014 */ //    inside of `UseTree`s, recording their `NodeId`s and grouping them by
/* FP:check_unused.rs-0015 */ //    the parent `use` item
/* FP:check_unused.rs-0016 */ //
/* FP:check_unused.rs-0017 */ //  - `calc_unused_spans` then walks over all the `use` items marked in the
/* FP:check_unused.rs-0018 */ //    previous step to collect the spans associated with the `NodeId`s and to
/* FP:check_unused.rs-0019 */ //    calculate the spans that can be removed by rustfix; This is done in a
/* FP:check_unused.rs-0020 */ //    separate step to be able to collapse the adjacent spans that rustfix
/* FP:check_unused.rs-0021 */ //    will remove
/* FP:check_unused.rs-0022 */ //
/* FP:check_unused.rs-0023 */ //  - `check_unused` finally emits the diagnostics based on the data generated
/* FP:check_unused.rs-0024 */ //    in the last step
/* FP:check_unused.rs-0025 */ 
/* FP:check_unused.rs-0026 */ use rustc_ast as ast;
/* FP:check_unused.rs-0027 */ use crate::rustc_complete::visit::{self, Visitor};
/* FP:check_unused.rs-0028 */ use crate::rustc_data_structures::fx::{FxHashMap, FxIndexMap, FxIndexSet};
/* FP:check_unused.rs-0029 */ use crate::rustc_data_structures::unord::UnordSet;
/* FP:check_unused.rs-0030 */ use crate::rustc_complete::MultiSpan;
/* FP:check_unused.rs-0031 */ use crate::rustc_complete::def::{DefKind, Res};
/* FP:check_unused.rs-0032 */ use crate::rustc_complete::lint::BuiltinLintDiag;
/* FP:check_unused.rs-0033 */ use crate::rustc_complete::lint::builtin::{
/* FP:check_unused.rs-0034 */     MACRO_USE_EXTERN_CRATE, UNUSED_EXTERN_CRATES, UNUSED_IMPORTS, UNUSED_QUALIFICATIONS,
/* FP:check_unused.rs-0035 */ };
/* FP:check_unused.rs-0036 */ use crate::rustc_complete::{DUMMY_SP, Ident, Macros20NormalizedIdent, Span, kw};
/* FP:check_unused.rs-0037 */ 
/* FP:check_unused.rs-0038 */ use crate::imports::{Import, ImportKind};
/* FP:check_unused.rs-0039 */ use crate::{LexicalScopeBinding, NameBindingKind, Resolver, module_to_string};
/* FP:check_unused.rs-0040 */ 
/* FP:check_unused.rs-0041 */ struct UnusedImport {
/* FP:check_unused.rs-0042 */     use_tree: ast::UseTree,
/* FP:check_unused.rs-0043 */     use_tree_id: ast::NodeId,
/* FP:check_unused.rs-0044 */     item_span: Span,
/* FP:check_unused.rs-0045 */     unused: UnordSet<ast::NodeId>,
/* FP:check_unused.rs-0046 */ }
/* FP:check_unused.rs-0047 */ 
/* FP:check_unused.rs-0048 */ impl UnusedImport {
/* FP:check_unused.rs-0049 */     fn add(&mut self, id: ast::NodeId) {
/* FP:check_unused.rs-0050 */         self.unused.insert(id);
/* FP:check_unused.rs-0051 */     }
/* FP:check_unused.rs-0052 */ }
/* FP:check_unused.rs-0053 */ 
/* FP:check_unused.rs-0054 */ struct UnusedImportCheckVisitor<'a, 'ra, 'tcx> {
/* FP:check_unused.rs-0055 */     r: &'a mut Resolver<'ra, 'tcx>,
/* FP:check_unused.rs-0056 */     /// All the (so far) unused imports, grouped path list
/* FP:check_unused.rs-0057 */     unused_imports: FxIndexMap<ast::NodeId, UnusedImport>,
/* FP:check_unused.rs-0058 */     extern_crate_items: Vec<ExternCrateToLint>,
/* FP:check_unused.rs-0059 */     base_use_tree: Option<&'a ast::UseTree>,
/* FP:check_unused.rs-0060 */     base_id: ast::NodeId,
/* FP:check_unused.rs-0061 */     item_span: Span,
/* FP:check_unused.rs-0062 */ }
/* FP:check_unused.rs-0063 */ 
/* FP:check_unused.rs-0064 */ struct ExternCrateToLint {
/* FP:check_unused.rs-0065 */     id: ast::NodeId,
/* FP:check_unused.rs-0066 */     /// Span from the item
/* FP:check_unused.rs-0067 */     span: Span,
/* FP:check_unused.rs-0068 */     /// Span to use to suggest complete removal.
/* FP:check_unused.rs-0069 */     span_with_attributes: Span,
/* FP:check_unused.rs-0070 */     /// Span of the visibility, if any.
/* FP:check_unused.rs-0071 */     vis_span: Span,
/* FP:check_unused.rs-0072 */     /// Whether the item has attrs.
/* FP:check_unused.rs-0073 */     has_attrs: bool,
/* FP:check_unused.rs-0074 */     /// Name used to refer to the crate.
/* FP:check_unused.rs-0075 */     ident: Ident,
/* FP:check_unused.rs-0076 */     /// Whether the statement renames the crate `extern crate orig_name as new_name;`.
/* FP:check_unused.rs-0077 */     renames: bool,
/* FP:check_unused.rs-0078 */ }
/* FP:check_unused.rs-0079 */ 
/* FP:check_unused.rs-0080 */ impl<'a, 'ra, 'tcx> UnusedImportCheckVisitor<'a, 'ra, 'tcx> {
/* FP:check_unused.rs-0081 */     // We have information about whether `use` (import) items are actually
/* FP:check_unused.rs-0082 */     // used now. If an import is not used at all, we signal a lint error.
/* FP:check_unused.rs-0083 */     fn check_import(&mut self, id: ast::NodeId) {
/* FP:check_unused.rs-0084 */         let used = self.r.used_imports.contains(&id);
/* FP:check_unused.rs-0085 */         let def_id = self.r.local_def_id(id);
/* FP:check_unused.rs-0086 */         if !used {
/* FP:check_unused.rs-0087 */             if self.r.maybe_unused_trait_imports.contains(&def_id) {
/* FP:check_unused.rs-0088 */                 // Check later.
/* FP:check_unused.rs-0089 */                 return;
/* FP:check_unused.rs-0090 */             }
/* FP:check_unused.rs-0091 */             self.unused_import(self.base_id).add(id);
/* FP:check_unused.rs-0092 */         } else {
/* FP:check_unused.rs-0093 */             // This trait import is definitely used, in a way other than
/* FP:check_unused.rs-0094 */             // method resolution.
/* FP:check_unused.rs-0095 */             // FIXME(#120456) - is `swap_remove` correct?
/* FP:check_unused.rs-0096 */             self.r.maybe_unused_trait_imports.swap_remove(&def_id);
/* FP:check_unused.rs-0097 */             if let Some(i) = self.unused_imports.get_mut(&self.base_id) {
/* FP:check_unused.rs-0098 */                 i.unused.remove(&id);
/* FP:check_unused.rs-0099 */             }
/* FP:check_unused.rs-0100 */         }
/* FP:check_unused.rs-0101 */     }
/* FP:check_unused.rs-0102 */ 
/* FP:check_unused.rs-0103 */     fn check_use_tree(&mut self, use_tree: &'a ast::UseTree, id: ast::NodeId) {
/* FP:check_unused.rs-0104 */         if self.r.effective_visibilities.is_exported(self.r.local_def_id(id)) {
/* FP:check_unused.rs-0105 */             self.check_import_as_underscore(use_tree, id);
/* FP:check_unused.rs-0106 */             return;
/* FP:check_unused.rs-0107 */         }
/* FP:check_unused.rs-0108 */ 
/* FP:check_unused.rs-0109 */         if let ast::UseTreeKind::Nested { ref items, .. } = use_tree.kind {
/* FP:check_unused.rs-0110 */             if items.is_empty() {
/* FP:check_unused.rs-0111 */                 self.unused_import(self.base_id).add(id);
/* FP:check_unused.rs-0112 */             }
/* FP:check_unused.rs-0113 */         } else {
/* FP:check_unused.rs-0114 */             self.check_import(id);
/* FP:check_unused.rs-0115 */         }
/* FP:check_unused.rs-0116 */     }
/* FP:check_unused.rs-0117 */ 
/* FP:check_unused.rs-0118 */     fn unused_import(&mut self, id: ast::NodeId) -> &mut UnusedImport {
/* FP:check_unused.rs-0119 */         let use_tree_id = self.base_id;
/* FP:check_unused.rs-0120 */         let use_tree = self.base_use_tree.unwrap().clone();
/* FP:check_unused.rs-0121 */         let item_span = self.item_span;
/* FP:check_unused.rs-0122 */ 
/* FP:check_unused.rs-0123 */         self.unused_imports.entry(id).or_insert_with(|| UnusedImport {
/* FP:check_unused.rs-0124 */             use_tree,
/* FP:check_unused.rs-0125 */             use_tree_id,
/* FP:check_unused.rs-0126 */             item_span,
/* FP:check_unused.rs-0127 */             unused: Default::default(),
/* FP:check_unused.rs-0128 */         })
/* FP:check_unused.rs-0129 */     }
/* FP:check_unused.rs-0130 */ 
/* FP:check_unused.rs-0131 */     fn check_import_as_underscore(&mut self, item: &ast::UseTree, id: ast::NodeId) {
/* FP:check_unused.rs-0132 */         match item.kind {
/* FP:check_unused.rs-0133 */             ast::UseTreeKind::Simple(Some(ident)) => {
/* FP:check_unused.rs-0134 */                 if ident.name == kw::Underscore
/* FP:check_unused.rs-0135 */                     && !self.r.import_res_map.get(&id).is_some_and(|per_ns| {
/* FP:check_unused.rs-0136 */                         matches!(
/* FP:check_unused.rs-0137 */                             per_ns.type_ns,
/* FP:check_unused.rs-0138 */                             Some(Res::Def(DefKind::Trait | DefKind::TraitAlias, _))
/* FP:check_unused.rs-0139 */                         )
/* FP:check_unused.rs-0140 */                     })
/* FP:check_unused.rs-0141 */                 {
/* FP:check_unused.rs-0142 */                     self.unused_import(self.base_id).add(id);
/* FP:check_unused.rs-0143 */                 }
/* FP:check_unused.rs-0144 */             }
/* FP:check_unused.rs-0145 */             ast::UseTreeKind::Nested { ref items, .. } => self.check_imports_as_underscore(items),
/* FP:check_unused.rs-0146 */             _ => {}
/* FP:check_unused.rs-0147 */         }
/* FP:check_unused.rs-0148 */     }
/* FP:check_unused.rs-0149 */ 
/* FP:check_unused.rs-0150 */     fn check_imports_as_underscore(&mut self, items: &[(ast::UseTree, ast::NodeId)]) {
/* FP:check_unused.rs-0151 */         for (item, id) in items {
/* FP:check_unused.rs-0152 */             self.check_import_as_underscore(item, *id);
/* FP:check_unused.rs-0153 */         }
/* FP:check_unused.rs-0154 */     }
/* FP:check_unused.rs-0155 */ 
/* FP:check_unused.rs-0156 */     fn report_unused_extern_crate_items(
/* FP:check_unused.rs-0157 */         &mut self,
/* FP:check_unused.rs-0158 */         maybe_unused_extern_crates: FxHashMap<ast::NodeId, Span>,
/* FP:check_unused.rs-0159 */     ) {
/* FP:check_unused.rs-0160 */         let tcx = self.r.tcx();
/* FP:check_unused.rs-0161 */         for extern_crate in &self.extern_crate_items {
/* FP:check_unused.rs-0162 */             let warn_if_unused = !extern_crate.ident.name.as_str().starts_with('_');
/* FP:check_unused.rs-0163 */ 
/* FP:check_unused.rs-0164 */             // If the crate is fully unused, we suggest removing it altogether.
/* FP:check_unused.rs-0165 */             // We do this in any edition.
/* FP:check_unused.rs-0166 */             if warn_if_unused {
/* FP:check_unused.rs-0167 */                 if let Some(&span) = maybe_unused_extern_crates.get(&extern_crate.id) {
/* FP:check_unused.rs-0168 */                     self.r.lint_buffer.buffer_lint(
/* FP:check_unused.rs-0169 */                         UNUSED_EXTERN_CRATES,
/* FP:check_unused.rs-0170 */                         extern_crate.id,
/* FP:check_unused.rs-0171 */                         span,
/* FP:check_unused.rs-0172 */                         BuiltinLintDiag::UnusedExternCrate {
/* FP:check_unused.rs-0173 */                             span: extern_crate.span,
/* FP:check_unused.rs-0174 */                             removal_span: extern_crate.span_with_attributes,
/* FP:check_unused.rs-0175 */                         },
/* FP:check_unused.rs-0176 */                     );
/* FP:check_unused.rs-0177 */                     continue;
/* FP:check_unused.rs-0178 */                 }
/* FP:check_unused.rs-0179 */             }
/* FP:check_unused.rs-0180 */ 
/* FP:check_unused.rs-0181 */             // If we are not in Rust 2018 edition, then we don't make any further
/* FP:check_unused.rs-0182 */             // suggestions.
/* FP:check_unused.rs-0183 */             if !tcx.sess.at_least_rust_2018() {
/* FP:check_unused.rs-0184 */                 continue;
/* FP:check_unused.rs-0185 */             }
/* FP:check_unused.rs-0186 */ 
/* FP:check_unused.rs-0187 */             // If the extern crate has any attributes, they may have funky
/* FP:check_unused.rs-0188 */             // semantics we can't faithfully represent using `use` (most
/* FP:check_unused.rs-0189 */             // notably `#[macro_use]`). Ignore it.
/* FP:check_unused.rs-0190 */             if extern_crate.has_attrs {
/* FP:check_unused.rs-0191 */                 continue;
/* FP:check_unused.rs-0192 */             }
/* FP:check_unused.rs-0193 */ 
/* FP:check_unused.rs-0194 */             // If the extern crate is renamed, then we cannot suggest replacing it with a use as this
/* FP:check_unused.rs-0195 */             // would not insert the new name into the prelude, where other imports in the crate may be
/* FP:check_unused.rs-0196 */             // expecting it.
/* FP:check_unused.rs-0197 */             if extern_crate.renames {
/* FP:check_unused.rs-0198 */                 continue;
/* FP:check_unused.rs-0199 */             }
/* FP:check_unused.rs-0200 */ 
/* FP:check_unused.rs-0201 */             // If the extern crate isn't in the extern prelude,
/* FP:check_unused.rs-0202 */             // there is no way it can be written as a `use`.
/* FP:check_unused.rs-0203 */             if self
/* FP:check_unused.rs-0204 */                 .r
/* FP:check_unused.rs-0205 */                 .extern_prelude
/* FP:check_unused.rs-0206 */                 .get(&Macros20NormalizedIdent::new(extern_crate.ident))
/* FP:check_unused.rs-0207 */                 .is_none_or(|entry| entry.introduced_by_item())
/* FP:check_unused.rs-0208 */             {
/* FP:check_unused.rs-0209 */                 continue;
/* FP:check_unused.rs-0210 */             }
/* FP:check_unused.rs-0211 */ 
/* FP:check_unused.rs-0212 */             let module = self
/* FP:check_unused.rs-0213 */                 .r
/* FP:check_unused.rs-0214 */                 .get_nearest_non_block_module(self.r.local_def_id(extern_crate.id).to_def_id());
/* FP:check_unused.rs-0215 */             if module.no_implicit_prelude {
/* FP:check_unused.rs-0216 */                 // If the module has `no_implicit_prelude`, then we don't suggest
/* FP:check_unused.rs-0217 */                 // replacing the extern crate with a use, as it would not be
/* FP:check_unused.rs-0218 */                 // inserted into the prelude. User writes `extern` style deliberately.
/* FP:check_unused.rs-0219 */                 continue;
/* FP:check_unused.rs-0220 */             }
/* FP:check_unused.rs-0221 */ 
/* FP:check_unused.rs-0222 */             let vis_span = extern_crate
/* FP:check_unused.rs-0223 */                 .vis_span
/* FP:check_unused.rs-0224 */                 .find_ancestor_inside(extern_crate.span)
/* FP:check_unused.rs-0225 */                 .unwrap_or(extern_crate.vis_span);
/* FP:check_unused.rs-0226 */             let ident_span = extern_crate
/* FP:check_unused.rs-0227 */                 .ident
/* FP:check_unused.rs-0228 */                 .span
/* FP:check_unused.rs-0229 */                 .find_ancestor_inside(extern_crate.span)
/* FP:check_unused.rs-0230 */                 .unwrap_or(extern_crate.ident.span);
/* FP:check_unused.rs-0231 */             self.r.lint_buffer.buffer_lint(
/* FP:check_unused.rs-0232 */                 UNUSED_EXTERN_CRATES,
/* FP:check_unused.rs-0233 */                 extern_crate.id,
/* FP:check_unused.rs-0234 */                 extern_crate.span,
/* FP:check_unused.rs-0235 */                 BuiltinLintDiag::ExternCrateNotIdiomatic { vis_span, ident_span },
/* FP:check_unused.rs-0236 */             );
/* FP:check_unused.rs-0237 */         }
/* FP:check_unused.rs-0238 */     }
/* FP:check_unused.rs-0239 */ }
/* FP:check_unused.rs-0240 */ 
/* FP:check_unused.rs-0241 */ impl<'a, 'ra, 'tcx> Visitor<'a> for UnusedImportCheckVisitor<'a, 'ra, 'tcx> {
/* FP:check_unused.rs-0242 */     fn visit_item(&mut self, item: &'a ast::Item) {
/* FP:check_unused.rs-0243 */         self.item_span = item.span_with_attributes();
/* FP:check_unused.rs-0244 */         match &item.kind {
/* FP:check_unused.rs-0245 */             // Ignore is_public import statements because there's no way to be sure
/* FP:check_unused.rs-0246 */             // whether they're used or not. Also ignore imports with a dummy span
/* FP:check_unused.rs-0247 */             // because this means that they were generated in some fashion by the
/* FP:check_unused.rs-0248 */             // compiler and we don't need to consider them.
/* FP:check_unused.rs-0249 */             ast::ItemKind::Use(..) if item.span.is_dummy() => return,
/* FP:check_unused.rs-0250 */             // Use the base UseTree's NodeId as the item id
/* FP:check_unused.rs-0251 */             // This allows the grouping of all the lints in the same item
/* FP:check_unused.rs-0252 */             ast::ItemKind::Use(use_tree) => {
/* FP:check_unused.rs-0253 */                 self.base_id = item.id;
/* FP:check_unused.rs-0254 */                 self.base_use_tree = Some(use_tree);
/* FP:check_unused.rs-0255 */                 self.check_use_tree(use_tree, item.id);
/* FP:check_unused.rs-0256 */             }
/* FP:check_unused.rs-0257 */             &ast::ItemKind::ExternCrate(orig_name, ident) => {
/* FP:check_unused.rs-0258 */                 self.extern_crate_items.push(ExternCrateToLint {
/* FP:check_unused.rs-0259 */                     id: item.id,
/* FP:check_unused.rs-0260 */                     span: item.span,
/* FP:check_unused.rs-0261 */                     vis_span: item.vis.span,
/* FP:check_unused.rs-0262 */                     span_with_attributes: item.span_with_attributes(),
/* FP:check_unused.rs-0263 */                     has_attrs: !item.attrs.is_empty(),
/* FP:check_unused.rs-0264 */                     ident,
/* FP:check_unused.rs-0265 */                     renames: orig_name.is_some(),
/* FP:check_unused.rs-0266 */                 });
/* FP:check_unused.rs-0267 */             }
/* FP:check_unused.rs-0268 */             _ => {}
/* FP:check_unused.rs-0269 */         }
/* FP:check_unused.rs-0270 */ 
/* FP:check_unused.rs-0271 */         visit::walk_item(self, item);
/* FP:check_unused.rs-0272 */     }
/* FP:check_unused.rs-0273 */ 
/* FP:check_unused.rs-0274 */     fn visit_nested_use_tree(&mut self, use_tree: &'a ast::UseTree, id: ast::NodeId) {
/* FP:check_unused.rs-0275 */         self.check_use_tree(use_tree, id);
/* FP:check_unused.rs-0276 */         visit::walk_use_tree(self, use_tree);
/* FP:check_unused.rs-0277 */     }
/* FP:check_unused.rs-0278 */ }
/* FP:check_unused.rs-0279 */ 
/* FP:check_unused.rs-0280 */ enum UnusedSpanResult {
/* FP:check_unused.rs-0281 */     Used,
/* FP:check_unused.rs-0282 */     Unused { spans: Vec<Span>, remove: Span },
/* FP:check_unused.rs-0283 */     PartialUnused { spans: Vec<Span>, remove: Vec<Span> },
/* FP:check_unused.rs-0284 */ }
/* FP:check_unused.rs-0285 */ 
/* FP:check_unused.rs-0286 */ fn calc_unused_spans(
/* FP:check_unused.rs-0287 */     unused_import: &UnusedImport,
/* FP:check_unused.rs-0288 */     use_tree: &ast::UseTree,
/* FP:check_unused.rs-0289 */     use_tree_id: ast::NodeId,
/* FP:check_unused.rs-0290 */ ) -> UnusedSpanResult {
/* FP:check_unused.rs-0291 */     // The full span is the whole item's span if this current tree is not nested inside another
/* FP:check_unused.rs-0292 */     // This tells rustfix to remove the whole item if all the imports are unused
/* FP:check_unused.rs-0293 */     let full_span = if unused_import.use_tree.span == use_tree.span {
/* FP:check_unused.rs-0294 */         unused_import.item_span
/* FP:check_unused.rs-0295 */     } else {
/* FP:check_unused.rs-0296 */         use_tree.span
/* FP:check_unused.rs-0297 */     };
/* FP:check_unused.rs-0298 */     match use_tree.kind {
/* FP:check_unused.rs-0299 */         ast::UseTreeKind::Simple(..) | ast::UseTreeKind::Glob => {
/* FP:check_unused.rs-0300 */             if unused_import.unused.contains(&use_tree_id) {
/* FP:check_unused.rs-0301 */                 UnusedSpanResult::Unused { spans: vec![use_tree.span], remove: full_span }
/* FP:check_unused.rs-0302 */             } else {
/* FP:check_unused.rs-0303 */                 UnusedSpanResult::Used
/* FP:check_unused.rs-0304 */             }
/* FP:check_unused.rs-0305 */         }
/* FP:check_unused.rs-0306 */         ast::UseTreeKind::Nested { items: ref nested, span: tree_span } => {
/* FP:check_unused.rs-0307 */             if nested.is_empty() {
/* FP:check_unused.rs-0308 */                 return UnusedSpanResult::Unused { spans: vec![use_tree.span], remove: full_span };
/* FP:check_unused.rs-0309 */             }
/* FP:check_unused.rs-0310 */ 
/* FP:check_unused.rs-0311 */             let mut unused_spans = Vec::new();
/* FP:check_unused.rs-0312 */             let mut to_remove = Vec::new();
/* FP:check_unused.rs-0313 */             let mut used_children = 0;
/* FP:check_unused.rs-0314 */             let mut contains_self = false;
/* FP:check_unused.rs-0315 */             let mut previous_unused = false;
/* FP:check_unused.rs-0316 */             for (pos, (use_tree, use_tree_id)) in nested.iter().enumerate() {
/* FP:check_unused.rs-0317 */                 let remove = match calc_unused_spans(unused_import, use_tree, *use_tree_id) {
/* FP:check_unused.rs-0318 */                     UnusedSpanResult::Used => {
/* FP:check_unused.rs-0319 */                         used_children += 1;
/* FP:check_unused.rs-0320 */                         None
/* FP:check_unused.rs-0321 */                     }
/* FP:check_unused.rs-0322 */                     UnusedSpanResult::Unused { mut spans, remove } => {
/* FP:check_unused.rs-0323 */                         unused_spans.append(&mut spans);
/* FP:check_unused.rs-0324 */                         Some(remove)
/* FP:check_unused.rs-0325 */                     }
/* FP:check_unused.rs-0326 */                     UnusedSpanResult::PartialUnused { mut spans, remove: mut to_remove_extra } => {
/* FP:check_unused.rs-0327 */                         used_children += 1;
/* FP:check_unused.rs-0328 */                         unused_spans.append(&mut spans);
/* FP:check_unused.rs-0329 */                         to_remove.append(&mut to_remove_extra);
/* FP:check_unused.rs-0330 */                         None
/* FP:check_unused.rs-0331 */                     }
/* FP:check_unused.rs-0332 */                 };
/* FP:check_unused.rs-0333 */                 if let Some(remove) = remove {
/* FP:check_unused.rs-0334 */                     let remove_span = if nested.len() == 1 {
/* FP:check_unused.rs-0335 */                         remove
/* FP:check_unused.rs-0336 */                     } else if pos == nested.len() - 1 || used_children > 0 {
/* FP:check_unused.rs-0337 */                         // Delete everything from the end of the last import, to delete the
/* FP:check_unused.rs-0338 */                         // previous comma
/* FP:check_unused.rs-0339 */                         nested[pos - 1].0.span.shrink_to_hi().to(use_tree.span)
/* FP:check_unused.rs-0340 */                     } else {
/* FP:check_unused.rs-0341 */                         // Delete everything until the next import, to delete the trailing commas
/* FP:check_unused.rs-0342 */                         use_tree.span.to(nested[pos + 1].0.span.shrink_to_lo())
/* FP:check_unused.rs-0343 */                     };
/* FP:check_unused.rs-0344 */ 
/* FP:check_unused.rs-0345 */                     // Try to collapse adjacent spans into a single one. This prevents all cases of
/* FP:check_unused.rs-0346 */                     // overlapping removals, which are not supported by rustfix
/* FP:check_unused.rs-0347 */                     if previous_unused && !to_remove.is_empty() {
/* FP:check_unused.rs-0348 */                         let previous = to_remove.pop().unwrap();
/* FP:check_unused.rs-0349 */                         to_remove.push(previous.to(remove_span));
/* FP:check_unused.rs-0350 */                     } else {
/* FP:check_unused.rs-0351 */                         to_remove.push(remove_span);
/* FP:check_unused.rs-0352 */                     }
/* FP:check_unused.rs-0353 */                 }
/* FP:check_unused.rs-0354 */                 contains_self |= use_tree.prefix == kw::SelfLower
/* FP:check_unused.rs-0355 */                     && matches!(use_tree.kind, ast::UseTreeKind::Simple(_))
/* FP:check_unused.rs-0356 */                     && !unused_import.unused.contains(&use_tree_id);
/* FP:check_unused.rs-0357 */                 previous_unused = remove.is_some();
/* FP:check_unused.rs-0358 */             }
/* FP:check_unused.rs-0359 */             if unused_spans.is_empty() {
/* FP:check_unused.rs-0360 */                 UnusedSpanResult::Used
/* FP:check_unused.rs-0361 */             } else if used_children == 0 {
/* FP:check_unused.rs-0362 */                 UnusedSpanResult::Unused { spans: unused_spans, remove: full_span }
/* FP:check_unused.rs-0363 */             } else {
/* FP:check_unused.rs-0364 */                 // If there is only one remaining child that is used, the braces around the use
/* FP:check_unused.rs-0365 */                 // tree are not needed anymore. In that case, we determine the span of the left
/* FP:check_unused.rs-0366 */                 // brace and the right brace, and tell rustfix to remove them as well.
/* FP:check_unused.rs-0367 */                 //
/* FP:check_unused.rs-0368 */                 // This means that `use a::{B, C};` will be turned into `use a::B;` rather than
/* FP:check_unused.rs-0369 */                 // `use a::{B};`, removing a rustfmt roundtrip.
/* FP:check_unused.rs-0370 */                 //
/* FP:check_unused.rs-0371 */                 // Note that we cannot remove the braces if the only item inside the use tree is
/* FP:check_unused.rs-0372 */                 // `self`: `use foo::{self};` is valid Rust syntax, while `use foo::self;` errors
/* FP:check_unused.rs-0373 */                 // out. We also cannot turn `use foo::{self}` into `use foo`, as the former doesn't
/* FP:check_unused.rs-0374 */                 // import types with the same name as the module.
/* FP:check_unused.rs-0375 */                 if used_children == 1 && !contains_self {
/* FP:check_unused.rs-0376 */                     // Left brace, from the start of the nested group to the first item.
/* FP:check_unused.rs-0377 */                     to_remove.push(
/* FP:check_unused.rs-0378 */                         tree_span.shrink_to_lo().to(nested.first().unwrap().0.span.shrink_to_lo()),
/* FP:check_unused.rs-0379 */                     );
/* FP:check_unused.rs-0380 */                     // Right brace, from the end of the last item to the end of the nested group.
/* FP:check_unused.rs-0381 */                     to_remove.push(
/* FP:check_unused.rs-0382 */                         nested.last().unwrap().0.span.shrink_to_hi().to(tree_span.shrink_to_hi()),
/* FP:check_unused.rs-0383 */                     );
/* FP:check_unused.rs-0384 */                 }
/* FP:check_unused.rs-0385 */ 
/* FP:check_unused.rs-0386 */                 UnusedSpanResult::PartialUnused { spans: unused_spans, remove: to_remove }
/* FP:check_unused.rs-0387 */             }
/* FP:check_unused.rs-0388 */         }
/* FP:check_unused.rs-0389 */     }
/* FP:check_unused.rs-0390 */ }
/* FP:check_unused.rs-0391 */ 
/* FP:check_unused.rs-0392 */ impl Resolver<'_, '_> {
/* FP:check_unused.rs-0393 */     pub(crate) fn check_unused(&mut self, krate: &ast::Crate) {
/* FP:check_unused.rs-0394 */         let tcx = self.tcx;
/* FP:check_unused.rs-0395 */         let mut maybe_unused_extern_crates = FxHashMap::default();
/* FP:check_unused.rs-0396 */ 
/* FP:check_unused.rs-0397 */         for import in self.potentially_unused_imports.iter() {
/* FP:check_unused.rs-0398 */             match import.kind {
/* FP:check_unused.rs-0399 */                 _ if import.vis.is_public()
/* FP:check_unused.rs-0400 */                     || import.span.is_dummy()
/* FP:check_unused.rs-0401 */                     || self.import_use_map.contains_key(import) =>
/* FP:check_unused.rs-0402 */                 {
/* FP:check_unused.rs-0403 */                     if let ImportKind::MacroUse { .. } = import.kind {
/* FP:check_unused.rs-0404 */                         if !import.span.is_dummy() {
/* FP:check_unused.rs-0405 */                             self.lint_buffer.buffer_lint(
/* FP:check_unused.rs-0406 */                                 MACRO_USE_EXTERN_CRATE,
/* FP:check_unused.rs-0407 */                                 import.root_id,
/* FP:check_unused.rs-0408 */                                 import.span,
/* FP:check_unused.rs-0409 */                                 BuiltinLintDiag::MacroUseDeprecated,
/* FP:check_unused.rs-0410 */                             );
/* FP:check_unused.rs-0411 */                         }
/* FP:check_unused.rs-0412 */                     }
/* FP:check_unused.rs-0413 */                 }
/* FP:check_unused.rs-0414 */                 ImportKind::ExternCrate { id, .. } => {
/* FP:check_unused.rs-0415 */                     let def_id = self.local_def_id(id);
/* FP:check_unused.rs-0416 */                     if self.extern_crate_map.get(&def_id).is_none_or(|&cnum| {
/* FP:check_unused.rs-0417 */                         !tcx.is_compiler_builtins(cnum)
/* FP:check_unused.rs-0418 */                             && !tcx.is_panic_runtime(cnum)
/* FP:check_unused.rs-0419 */                             && !tcx.has_global_allocator(cnum)
/* FP:check_unused.rs-0420 */                             && !tcx.has_panic_handler(cnum)
/* FP:check_unused.rs-0421 */                     }) {
/* FP:check_unused.rs-0422 */                         maybe_unused_extern_crates.insert(id, import.span);
/* FP:check_unused.rs-0423 */                     }
/* FP:check_unused.rs-0424 */                 }
/* FP:check_unused.rs-0425 */                 ImportKind::MacroUse { .. } => {
/* FP:check_unused.rs-0426 */                     self.lint_buffer.buffer_lint(
/* FP:check_unused.rs-0427 */                         UNUSED_IMPORTS,
/* FP:check_unused.rs-0428 */                         import.root_id,
/* FP:check_unused.rs-0429 */                         import.span,
/* FP:check_unused.rs-0430 */                         BuiltinLintDiag::UnusedMacroUse,
/* FP:check_unused.rs-0431 */                     );
/* FP:check_unused.rs-0432 */                 }
/* FP:check_unused.rs-0433 */                 _ => {}
/* FP:check_unused.rs-0434 */             }
/* FP:check_unused.rs-0435 */         }
/* FP:check_unused.rs-0436 */ 
/* FP:check_unused.rs-0437 */         let mut visitor = UnusedImportCheckVisitor {
/* FP:check_unused.rs-0438 */             r: self,
/* FP:check_unused.rs-0439 */             unused_imports: Default::default(),
/* FP:check_unused.rs-0440 */             extern_crate_items: Default::default(),
/* FP:check_unused.rs-0441 */             base_use_tree: None,
/* FP:check_unused.rs-0442 */             base_id: ast::DUMMY_NODE_ID,
/* FP:check_unused.rs-0443 */             item_span: DUMMY_SP,
/* FP:check_unused.rs-0444 */         };
/* FP:check_unused.rs-0445 */         visit::walk_crate(&mut visitor, krate);
/* FP:check_unused.rs-0446 */ 
/* FP:check_unused.rs-0447 */         visitor.report_unused_extern_crate_items(maybe_unused_extern_crates);
/* FP:check_unused.rs-0448 */ 
/* FP:check_unused.rs-0449 */         for unused in visitor.unused_imports.values() {
/* FP:check_unused.rs-0450 */             let (spans, remove_spans) =
/* FP:check_unused.rs-0451 */                 match calc_unused_spans(unused, &unused.use_tree, unused.use_tree_id) {
/* FP:check_unused.rs-0452 */                     UnusedSpanResult::Used => continue,
/* FP:check_unused.rs-0453 */                     UnusedSpanResult::Unused { spans, remove } => (spans, vec![remove]),
/* FP:check_unused.rs-0454 */                     UnusedSpanResult::PartialUnused { spans, remove } => (spans, remove),
/* FP:check_unused.rs-0455 */                 };
/* FP:check_unused.rs-0456 */ 
/* FP:check_unused.rs-0457 */             let ms = MultiSpan::from_spans(spans);
/* FP:check_unused.rs-0458 */ 
/* FP:check_unused.rs-0459 */             let mut span_snippets = ms
/* FP:check_unused.rs-0460 */                 .primary_spans()
/* FP:check_unused.rs-0461 */                 .iter()
/* FP:check_unused.rs-0462 */                 .filter_map(|span| tcx.sess.source_map().span_to_snippet(*span).ok())
/* FP:check_unused.rs-0463 */                 .map(|s| format!("`{s}`"))
/* FP:check_unused.rs-0464 */                 .collect::<Vec<String>>();
/* FP:check_unused.rs-0465 */             span_snippets.sort();
/* FP:check_unused.rs-0466 */ 
/* FP:check_unused.rs-0467 */             let remove_whole_use = remove_spans.len() == 1 && remove_spans[0] == unused.item_span;
/* FP:check_unused.rs-0468 */             let num_to_remove = ms.primary_spans().len();
/* FP:check_unused.rs-0469 */ 
/* FP:check_unused.rs-0470 */             // If we are in the `--test` mode, suppress a help that adds the `#[cfg(test)]`
/* FP:check_unused.rs-0471 */             // attribute; however, if not, suggest adding the attribute. There is no way to
/* FP:check_unused.rs-0472 */             // retrieve attributes here because we do not have a `TyCtxt` yet.
/* FP:check_unused.rs-0473 */             let test_module_span = if tcx.sess.is_test_crate() {
/* FP:check_unused.rs-0474 */                 None
/* FP:check_unused.rs-0475 */             } else {
/* FP:check_unused.rs-0476 */                 let parent_module = visitor.r.get_nearest_non_block_module(
/* FP:check_unused.rs-0477 */                     visitor.r.local_def_id(unused.use_tree_id).to_def_id(),
/* FP:check_unused.rs-0478 */                 );
/* FP:check_unused.rs-0479 */                 match module_to_string(parent_module) {
/* FP:check_unused.rs-0480 */                     Some(module)
/* FP:check_unused.rs-0481 */                         if module == "test"
/* FP:check_unused.rs-0482 */                             || module == "tests"
/* FP:check_unused.rs-0483 */                             || module.starts_with("test_")
/* FP:check_unused.rs-0484 */                             || module.starts_with("tests_")
/* FP:check_unused.rs-0485 */                             || module.ends_with("_test")
/* FP:check_unused.rs-0486 */                             || module.ends_with("_tests") =>
/* FP:check_unused.rs-0487 */                     {
/* FP:check_unused.rs-0488 */                         Some(parent_module.span)
/* FP:check_unused.rs-0489 */                     }
/* FP:check_unused.rs-0490 */                     _ => None,
/* FP:check_unused.rs-0491 */                 }
/* FP:check_unused.rs-0492 */             };
/* FP:check_unused.rs-0493 */ 
/* FP:check_unused.rs-0494 */             visitor.r.lint_buffer.buffer_lint(
/* FP:check_unused.rs-0495 */                 UNUSED_IMPORTS,
/* FP:check_unused.rs-0496 */                 unused.use_tree_id,
/* FP:check_unused.rs-0497 */                 ms,
/* FP:check_unused.rs-0498 */                 BuiltinLintDiag::UnusedImports {
/* FP:check_unused.rs-0499 */                     remove_whole_use,
/* FP:check_unused.rs-0500 */                     num_to_remove,
/* FP:check_unused.rs-0501 */                     remove_spans,
/* FP:check_unused.rs-0502 */                     test_module_span,
/* FP:check_unused.rs-0503 */                     span_snippets,
/* FP:check_unused.rs-0504 */                 },
/* FP:check_unused.rs-0505 */             );
/* FP:check_unused.rs-0506 */         }
/* FP:check_unused.rs-0507 */ 
/* FP:check_unused.rs-0508 */         let unused_imports = visitor.unused_imports;
/* FP:check_unused.rs-0509 */         let mut check_redundant_imports = FxIndexSet::default();
/* FP:check_unused.rs-0510 */         for module in self.arenas.local_modules().iter() {
/* FP:check_unused.rs-0511 */             for (_key, resolution) in self.resolutions(*module).borrow().iter() {
/* FP:check_unused.rs-0512 */                 if let Some(binding) = resolution.borrow().best_binding()
/* FP:check_unused.rs-0513 */                     && let NameBindingKind::Import { import, .. } = binding.kind
/* FP:check_unused.rs-0514 */                     && let ImportKind::Single { id, .. } = import.kind
/* FP:check_unused.rs-0515 */                 {
/* FP:check_unused.rs-0516 */                     if let Some(unused_import) = unused_imports.get(&import.root_id)
/* FP:check_unused.rs-0517 */                         && unused_import.unused.contains(&id)
/* FP:check_unused.rs-0518 */                     {
/* FP:check_unused.rs-0519 */                         continue;
/* FP:check_unused.rs-0520 */                     }
/* FP:check_unused.rs-0521 */ 
/* FP:check_unused.rs-0522 */                     check_redundant_imports.insert(import);
/* FP:check_unused.rs-0523 */                 }
/* FP:check_unused.rs-0524 */             }
/* FP:check_unused.rs-0525 */         }
/* FP:check_unused.rs-0526 */ 
/* FP:check_unused.rs-0527 */         let mut redundant_imports = UnordSet::default();
/* FP:check_unused.rs-0528 */         for import in check_redundant_imports {
/* FP:check_unused.rs-0529 */             if self.check_for_redundant_imports(import)
/* FP:check_unused.rs-0530 */                 && let Some(id) = import.id()
/* FP:check_unused.rs-0531 */             {
/* FP:check_unused.rs-0532 */                 redundant_imports.insert(id);
/* FP:check_unused.rs-0533 */             }
/* FP:check_unused.rs-0534 */         }
/* FP:check_unused.rs-0535 */ 
/* FP:check_unused.rs-0536 */         // The lint fixes for unused_import and unnecessary_qualification may conflict.
/* FP:check_unused.rs-0537 */         // Deleting both unused imports and unnecessary segments of an item may result
/* FP:check_unused.rs-0538 */         // in the item not being found.
/* FP:check_unused.rs-0539 */         for unn_qua in &self.potentially_unnecessary_qualifications {
/* FP:check_unused.rs-0540 */             if let LexicalScopeBinding::Item(name_binding) = unn_qua.binding
/* FP:check_unused.rs-0541 */                 && let NameBindingKind::Import { import, .. } = name_binding.kind
/* FP:check_unused.rs-0542 */                 && (is_unused_import(import, &unused_imports)
/* FP:check_unused.rs-0543 */                     || is_redundant_import(import, &redundant_imports))
/* FP:check_unused.rs-0544 */             {
/* FP:check_unused.rs-0545 */                 continue;
/* FP:check_unused.rs-0546 */             }
/* FP:check_unused.rs-0547 */ 
/* FP:check_unused.rs-0548 */             self.lint_buffer.buffer_lint(
/* FP:check_unused.rs-0549 */                 UNUSED_QUALIFICATIONS,
/* FP:check_unused.rs-0550 */                 unn_qua.node_id,
/* FP:check_unused.rs-0551 */                 unn_qua.path_span,
/* FP:check_unused.rs-0552 */                 BuiltinLintDiag::UnusedQualifications { removal_span: unn_qua.removal_span },
/* FP:check_unused.rs-0553 */             );
/* FP:check_unused.rs-0554 */         }
/* FP:check_unused.rs-0555 */ 
/* FP:check_unused.rs-0556 */         fn is_redundant_import(
/* FP:check_unused.rs-0557 */             import: Import<'_>,
/* FP:check_unused.rs-0558 */             redundant_imports: &UnordSet<ast::NodeId>,
/* FP:check_unused.rs-0559 */         ) -> bool {
/* FP:check_unused.rs-0560 */             if let Some(id) = import.id()
/* FP:check_unused.rs-0561 */                 && redundant_imports.contains(&id)
/* FP:check_unused.rs-0562 */             {
/* FP:check_unused.rs-0563 */                 return true;
/* FP:check_unused.rs-0564 */             }
/* FP:check_unused.rs-0565 */             false
/* FP:check_unused.rs-0566 */         }
/* FP:check_unused.rs-0567 */ 
/* FP:check_unused.rs-0568 */         fn is_unused_import(
/* FP:check_unused.rs-0569 */             import: Import<'_>,
/* FP:check_unused.rs-0570 */             unused_imports: &FxIndexMap<ast::NodeId, UnusedImport>,
/* FP:check_unused.rs-0571 */         ) -> bool {
/* FP:check_unused.rs-0572 */             if let Some(unused_import) = unused_imports.get(&import.root_id)
/* FP:check_unused.rs-0573 */                 && let Some(id) = import.id()
/* FP:check_unused.rs-0574 */                 && unused_import.unused.contains(&id)
/* FP:check_unused.rs-0575 */             {
/* FP:check_unused.rs-0576 */                 return true;
/* FP:check_unused.rs-0577 */             }
/* FP:check_unused.rs-0578 */             false
/* FP:check_unused.rs-0579 */         }
/* FP:check_unused.rs-0580 */     }
/* FP:check_unused.rs-0581 */ }