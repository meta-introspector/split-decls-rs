/* FP:test.rs-0001 */ // The expansion from a test function to the appropriate test struct for libtest
/* FP:test.rs-0002 */ // Ideally, this code would be in libtest but for efficiency and error messages it lives here.
/* FP:test.rs-0003 */ 
/* FP:test.rs-0004 */ use std::assert_matches::assert_matches;
/* FP:test.rs-0005 */ use std::iter;
/* FP:test.rs-0006 */ 
/* FP:test.rs-0007 */ use crate::rustc_complete::{self as ast, GenericParamKind, HasNodeId, attr, join_path_idents};
/* FP:test.rs-0008 */ use rustc_ast_pretty::pprust;
/* FP:test.rs-0009 */ use rustc_attr_parsing::AttributeParser;
/* FP:test.rs-0010 */ use crate::rustc_complete::{Applicability, Diag, Level};
/* FP:test.rs-0011 */ use crate::rustc_expand::base::*;
/* FP:test.rs-0012 */ use crate::rustc_complete::Attribute;
/* FP:test.rs-0013 */ use crate::rustc_complete::attrs::AttributeKind;
/* FP:test.rs-0014 */ use crate::rustc_complete::{ErrorGuaranteed, FileNameDisplayPreference, Ident, Span, Symbol, sym};
/* FP:test.rs-0015 */ use thin_vec::{ThinVec, thin_vec};
/* FP:test.rs-0016 */ use tracing::debug;
/* FP:test.rs-0017 */ 
/* FP:test.rs-0018 */ use crate::errors;
/* FP:test.rs-0019 */ use crate::util::{check_builtin_macro_attribute, warn_on_duplicate_attribute};
/* FP:test.rs-0020 */ 
/* FP:test.rs-0021 */ /// #[test_case] is used by custom test authors to mark tests
/* FP:test.rs-0022 */ /// When building for test, it needs to make the item public and gensym the name
/* FP:test.rs-0023 */ /// Otherwise, we'll omit the item. This behavior means that any item annotated
/* FP:test.rs-0024 */ /// with #[test_case] is never addressable.
/* FP:test.rs-0025 */ ///
/* FP:test.rs-0026 */ /// We mark item with an inert attribute "rustc_test_marker" which the test generation
/* FP:test.rs-0027 */ /// logic will pick up on.
/* FP:test.rs-0028 */ pub(crate) fn expand_test_case(
/* FP:test.rs-0029 */     ecx: &mut ExtCtxt<'_>,
/* FP:test.rs-0030 */     attr_sp: Span,
/* FP:test.rs-0031 */     meta_item: &ast::MetaItem,
/* FP:test.rs-0032 */     anno_item: Annotatable,
/* FP:test.rs-0033 */ ) -> Vec<Annotatable> {
/* FP:test.rs-0034 */     check_builtin_macro_attribute(ecx, meta_item, sym::test_case);
/* FP:test.rs-0035 */     warn_on_duplicate_attribute(ecx, &anno_item, sym::test_case);
/* FP:test.rs-0036 */ 
/* FP:test.rs-0037 */     if !ecx.ecfg.should_test {
/* FP:test.rs-0038 */         return vec![];
/* FP:test.rs-0039 */     }
/* FP:test.rs-0040 */ 
/* FP:test.rs-0041 */     let sp = ecx.with_def_site_ctxt(attr_sp);
/* FP:test.rs-0042 */     let (mut item, is_stmt) = match anno_item {
/* FP:test.rs-0043 */         Annotatable::Item(item) => (item, false),
/* FP:test.rs-0044 */         Annotatable::Stmt(stmt) if let ast::StmtKind::Item(_) = stmt.kind => {
/* FP:test.rs-0045 */             if let ast::StmtKind::Item(i) = stmt.kind {
/* FP:test.rs-0046 */                 (i, true)
/* FP:test.rs-0047 */             } else {
/* FP:test.rs-0048 */                 unreachable!()
/* FP:test.rs-0049 */             }
/* FP:test.rs-0050 */         }
/* FP:test.rs-0051 */         _ => {
/* FP:test.rs-0052 */             ecx.dcx().emit_err(errors::TestCaseNonItem { span: anno_item.span() });
/* FP:test.rs-0053 */             return vec![];
/* FP:test.rs-0054 */         }
/* FP:test.rs-0055 */     };
/* FP:test.rs-0056 */ 
/* FP:test.rs-0057 */     // `#[test_case]` is valid on functions, consts, and statics. Only modify
/* FP:test.rs-0058 */     // the item in those cases.
/* FP:test.rs-0059 */     match &mut item.kind {
/* FP:test.rs-0060 */         ast::ItemKind::Fn(box ast::Fn { ident, .. })
/* FP:test.rs-0061 */         | ast::ItemKind::Const(box ast::ConstItem { ident, .. })
/* FP:test.rs-0062 */         | ast::ItemKind::Static(box ast::StaticItem { ident, .. }) => {
/* FP:test.rs-0063 */             ident.span = ident.span.with_ctxt(sp.ctxt());
/* FP:test.rs-0064 */             let test_path_symbol = Symbol::intern(&item_path(
/* FP:test.rs-0065 */                 // skip the name of the root module
/* FP:test.rs-0066 */                 &ecx.current_expansion.module.mod_path[1..],
/* FP:test.rs-0067 */                 ident,
/* FP:test.rs-0068 */             ));
/* FP:test.rs-0069 */             item.vis = ast::Visibility {
/* FP:test.rs-0070 */                 span: item.vis.span,
/* FP:test.rs-0071 */                 kind: ast::VisibilityKind::Public,
/* FP:test.rs-0072 */                 tokens: None,
/* FP:test.rs-0073 */             };
/* FP:test.rs-0074 */             item.attrs.push(ecx.attr_name_value_str(sym::rustc_test_marker, test_path_symbol, sp));
/* FP:test.rs-0075 */         }
/* FP:test.rs-0076 */         _ => {}
/* FP:test.rs-0077 */     }
/* FP:test.rs-0078 */ 
/* FP:test.rs-0079 */     let ret = if is_stmt {
/* FP:test.rs-0080 */         Annotatable::Stmt(Box::new(ecx.stmt_item(item.span, item)))
/* FP:test.rs-0081 */     } else {
/* FP:test.rs-0082 */         Annotatable::Item(item)
/* FP:test.rs-0083 */     };
/* FP:test.rs-0084 */ 
/* FP:test.rs-0085 */     vec![ret]
/* FP:test.rs-0086 */ }
/* FP:test.rs-0087 */ 
/* FP:test.rs-0088 */ pub(crate) fn expand_test(
/* FP:test.rs-0089 */     cx: &mut ExtCtxt<'_>,
/* FP:test.rs-0090 */     attr_sp: Span,
/* FP:test.rs-0091 */     meta_item: &ast::MetaItem,
/* FP:test.rs-0092 */     item: Annotatable,
/* FP:test.rs-0093 */ ) -> Vec<Annotatable> {
/* FP:test.rs-0094 */     check_builtin_macro_attribute(cx, meta_item, sym::test);
/* FP:test.rs-0095 */     warn_on_duplicate_attribute(cx, &item, sym::test);
/* FP:test.rs-0096 */     expand_test_or_bench(cx, attr_sp, item, false)
/* FP:test.rs-0097 */ }
/* FP:test.rs-0098 */ 
/* FP:test.rs-0099 */ pub(crate) fn expand_bench(
/* FP:test.rs-0100 */     cx: &mut ExtCtxt<'_>,
/* FP:test.rs-0101 */     attr_sp: Span,
/* FP:test.rs-0102 */     meta_item: &ast::MetaItem,
/* FP:test.rs-0103 */     item: Annotatable,
/* FP:test.rs-0104 */ ) -> Vec<Annotatable> {
/* FP:test.rs-0105 */     check_builtin_macro_attribute(cx, meta_item, sym::bench);
/* FP:test.rs-0106 */     warn_on_duplicate_attribute(cx, &item, sym::bench);
/* FP:test.rs-0107 */     expand_test_or_bench(cx, attr_sp, item, true)
/* FP:test.rs-0108 */ }
/* FP:test.rs-0109 */ 
/* FP:test.rs-0110 */ pub(crate) fn expand_test_or_bench(
/* FP:test.rs-0111 */     cx: &ExtCtxt<'_>,
/* FP:test.rs-0112 */     attr_sp: Span,
/* FP:test.rs-0113 */     item: Annotatable,
/* FP:test.rs-0114 */     is_bench: bool,
/* FP:test.rs-0115 */ ) -> Vec<Annotatable> {
/* FP:test.rs-0116 */     // If we're not in test configuration, remove the annotated item
/* FP:test.rs-0117 */     if !cx.ecfg.should_test {
/* FP:test.rs-0118 */         return vec![];
/* FP:test.rs-0119 */     }
/* FP:test.rs-0120 */ 
/* FP:test.rs-0121 */     let (item, is_stmt) = match item {
/* FP:test.rs-0122 */         Annotatable::Item(i) => (i, false),
/* FP:test.rs-0123 */         Annotatable::Stmt(box ast::Stmt { kind: ast::StmtKind::Item(i), .. }) => (i, true),
/* FP:test.rs-0124 */         other => {
/* FP:test.rs-0125 */             not_testable_error(cx, attr_sp, None);
/* FP:test.rs-0126 */             return vec![other];
/* FP:test.rs-0127 */         }
/* FP:test.rs-0128 */     };
/* FP:test.rs-0129 */ 
/* FP:test.rs-0130 */     let ast::ItemKind::Fn(fn_) = &item.kind else {
/* FP:test.rs-0131 */         not_testable_error(cx, attr_sp, Some(&item));
/* FP:test.rs-0132 */         return if is_stmt {
/* FP:test.rs-0133 */             vec![Annotatable::Stmt(Box::new(cx.stmt_item(item.span, item)))]
/* FP:test.rs-0134 */         } else {
/* FP:test.rs-0135 */             vec![Annotatable::Item(item)]
/* FP:test.rs-0136 */         };
/* FP:test.rs-0137 */     };
/* FP:test.rs-0138 */ 
/* FP:test.rs-0139 */     if let Some(attr) = attr::find_by_name(&item.attrs, sym::naked) {
/* FP:test.rs-0140 */         cx.dcx().emit_err(errors::NakedFunctionTestingAttribute {
/* FP:test.rs-0141 */             testing_span: attr_sp,
/* FP:test.rs-0142 */             naked_span: attr.span,
/* FP:test.rs-0143 */         });
/* FP:test.rs-0144 */         return vec![Annotatable::Item(item)];
/* FP:test.rs-0145 */     }
/* FP:test.rs-0146 */ 
/* FP:test.rs-0147 */     // check_*_signature will report any errors in the type so compilation
/* FP:test.rs-0148 */     // will fail. We shouldn't try to expand in this case because the errors
/* FP:test.rs-0149 */     // would be spurious.
/* FP:test.rs-0150 */     let check_result = if is_bench {
/* FP:test.rs-0151 */         check_bench_signature(cx, &item, fn_)
/* FP:test.rs-0152 */     } else {
/* FP:test.rs-0153 */         check_test_signature(cx, &item, fn_)
/* FP:test.rs-0154 */     };
/* FP:test.rs-0155 */     if check_result.is_err() {
/* FP:test.rs-0156 */         return if is_stmt {
/* FP:test.rs-0157 */             vec![Annotatable::Stmt(Box::new(cx.stmt_item(item.span, item)))]
/* FP:test.rs-0158 */         } else {
/* FP:test.rs-0159 */             vec![Annotatable::Item(item)]
/* FP:test.rs-0160 */         };
/* FP:test.rs-0161 */     }
/* FP:test.rs-0162 */ 
/* FP:test.rs-0163 */     let sp = cx.with_def_site_ctxt(item.span);
/* FP:test.rs-0164 */     let ret_ty_sp = cx.with_def_site_ctxt(fn_.sig.decl.output.span());
/* FP:test.rs-0165 */     let attr_sp = cx.with_def_site_ctxt(attr_sp);
/* FP:test.rs-0166 */ 
/* FP:test.rs-0167 */     let test_ident = Ident::new(sym::test, attr_sp);
/* FP:test.rs-0168 */ 
/* FP:test.rs-0169 */     // creates test::$name
/* FP:test.rs-0170 */     let test_path = |name| cx.path(ret_ty_sp, vec![test_ident, Ident::from_str_and_span(name, sp)]);
/* FP:test.rs-0171 */ 
/* FP:test.rs-0172 */     // creates test::ShouldPanic::$name
/* FP:test.rs-0173 */     let should_panic_path = |name| {
/* FP:test.rs-0174 */         cx.path(
/* FP:test.rs-0175 */             sp,
/* FP:test.rs-0176 */             vec![
/* FP:test.rs-0177 */                 test_ident,
/* FP:test.rs-0178 */                 Ident::from_str_and_span("ShouldPanic", sp),
/* FP:test.rs-0179 */                 Ident::from_str_and_span(name, sp),
/* FP:test.rs-0180 */             ],
/* FP:test.rs-0181 */         )
/* FP:test.rs-0182 */     };
/* FP:test.rs-0183 */ 
/* FP:test.rs-0184 */     // creates test::TestType::$name
/* FP:test.rs-0185 */     let test_type_path = |name| {
/* FP:test.rs-0186 */         cx.path(
/* FP:test.rs-0187 */             sp,
/* FP:test.rs-0188 */             vec![
/* FP:test.rs-0189 */                 test_ident,
/* FP:test.rs-0190 */                 Ident::from_str_and_span("TestType", sp),
/* FP:test.rs-0191 */                 Ident::from_str_and_span(name, sp),
/* FP:test.rs-0192 */             ],
/* FP:test.rs-0193 */         )
/* FP:test.rs-0194 */     };
/* FP:test.rs-0195 */ 
/* FP:test.rs-0196 */     // creates $name: $expr
/* FP:test.rs-0197 */     let field = |name, expr| cx.field_imm(sp, Ident::from_str_and_span(name, sp), expr);
/* FP:test.rs-0198 */ 
/* FP:test.rs-0199 */     // Adds `#[coverage(off)]` to a closure, so it won't be instrumented in
/* FP:test.rs-0200 */     // `-Cinstrument-coverage` builds.
/* FP:test.rs-0201 */     // This requires `#[allow_internal_unstable(coverage_attribute)]` on the
/* FP:test.rs-0202 */     // corresponding macro declaration in `core::macros`.
/* FP:test.rs-0203 */     let coverage_off = |mut expr: Box<ast::Expr>| {
/* FP:test.rs-0204 */         assert_matches!(expr.kind, ast::ExprKind::Closure(_));
/* FP:test.rs-0205 */         expr.attrs.push(cx.attr_nested_word(sym::coverage, sym::off, sp));
/* FP:test.rs-0206 */         expr
/* FP:test.rs-0207 */     };
/* FP:test.rs-0208 */ 
/* FP:test.rs-0209 */     let test_fn = if is_bench {
/* FP:test.rs-0210 */         // A simple ident for a lambda
/* FP:test.rs-0211 */         let b = Ident::from_str_and_span("b", attr_sp);
/* FP:test.rs-0212 */ 
/* FP:test.rs-0213 */         cx.expr_call(
/* FP:test.rs-0214 */             sp,
/* FP:test.rs-0215 */             cx.expr_path(test_path("StaticBenchFn")),
/* FP:test.rs-0216 */             thin_vec![
/* FP:test.rs-0217 */                 // #[coverage(off)]
/* FP:test.rs-0218 */                 // |b| self::test::assert_test_result(
/* FP:test.rs-0219 */                 coverage_off(cx.lambda1(
/* FP:test.rs-0220 */                     sp,
/* FP:test.rs-0221 */                     cx.expr_call(
/* FP:test.rs-0222 */                         sp,
/* FP:test.rs-0223 */                         cx.expr_path(test_path("assert_test_result")),
/* FP:test.rs-0224 */                         thin_vec![
/* FP:test.rs-0225 */                             // super::$test_fn(b)
/* FP:test.rs-0226 */                             cx.expr_call(
/* FP:test.rs-0227 */                                 ret_ty_sp,
/* FP:test.rs-0228 */                                 cx.expr_path(cx.path(sp, vec![fn_.ident])),
/* FP:test.rs-0229 */                                 thin_vec![cx.expr_ident(sp, b)],
/* FP:test.rs-0230 */                             ),
/* FP:test.rs-0231 */                         ],
/* FP:test.rs-0232 */                     ),
/* FP:test.rs-0233 */                     b,
/* FP:test.rs-0234 */                 )), // )
/* FP:test.rs-0235 */             ],
/* FP:test.rs-0236 */         )
/* FP:test.rs-0237 */     } else {
/* FP:test.rs-0238 */         cx.expr_call(
/* FP:test.rs-0239 */             sp,
/* FP:test.rs-0240 */             cx.expr_path(test_path("StaticTestFn")),
/* FP:test.rs-0241 */             thin_vec![
/* FP:test.rs-0242 */                 // #[coverage(off)]
/* FP:test.rs-0243 */                 // || {
/* FP:test.rs-0244 */                 coverage_off(cx.lambda0(
/* FP:test.rs-0245 */                     sp,
/* FP:test.rs-0246 */                     // test::assert_test_result(
/* FP:test.rs-0247 */                     cx.expr_call(
/* FP:test.rs-0248 */                         sp,
/* FP:test.rs-0249 */                         cx.expr_path(test_path("assert_test_result")),
/* FP:test.rs-0250 */                         thin_vec![
/* FP:test.rs-0251 */                             // $test_fn()
/* FP:test.rs-0252 */                             cx.expr_call(
/* FP:test.rs-0253 */                                 ret_ty_sp,
/* FP:test.rs-0254 */                                 cx.expr_path(cx.path(sp, vec![fn_.ident])),
/* FP:test.rs-0255 */                                 ThinVec::new(),
/* FP:test.rs-0256 */                             ), // )
/* FP:test.rs-0257 */                         ],
/* FP:test.rs-0258 */                     ), // }
/* FP:test.rs-0259 */                 )), // )
/* FP:test.rs-0260 */             ],
/* FP:test.rs-0261 */         )
/* FP:test.rs-0262 */     };
/* FP:test.rs-0263 */ 
/* FP:test.rs-0264 */     let test_path_symbol = Symbol::intern(&item_path(
/* FP:test.rs-0265 */         // skip the name of the root module
/* FP:test.rs-0266 */         &cx.current_expansion.module.mod_path[1..],
/* FP:test.rs-0267 */         &fn_.ident,
/* FP:test.rs-0268 */     ));
/* FP:test.rs-0269 */ 
/* FP:test.rs-0270 */     let location_info = get_location_info(cx, &fn_);
/* FP:test.rs-0271 */ 
/* FP:test.rs-0272 */     let mut test_const =
/* FP:test.rs-0273 */         cx.item(
/* FP:test.rs-0274 */             sp,
/* FP:test.rs-0275 */             thin_vec![
/* FP:test.rs-0276 */                 // #[cfg(test)]
/* FP:test.rs-0277 */                 cx.attr_nested_word(sym::cfg, sym::test, attr_sp),
/* FP:test.rs-0278 */                 // #[rustc_test_marker = "test_case_sort_key"]
/* FP:test.rs-0279 */                 cx.attr_name_value_str(sym::rustc_test_marker, test_path_symbol, attr_sp),
/* FP:test.rs-0280 */                 // #[doc(hidden)]
/* FP:test.rs-0281 */                 cx.attr_nested_word(sym::doc, sym::hidden, attr_sp),
/* FP:test.rs-0282 */             ],
/* FP:test.rs-0283 */             // const $ident: test::TestDescAndFn =
/* FP:test.rs-0284 */             ast::ItemKind::Const(
/* FP:test.rs-0285 */                 ast::ConstItem {
/* FP:test.rs-0286 */                     defaultness: ast::Defaultness::Final,
/* FP:test.rs-0287 */                     ident: Ident::new(fn_.ident.name, sp),
/* FP:test.rs-0288 */                     generics: ast::Generics::default(),
/* FP:test.rs-0289 */                     ty: cx.ty(sp, ast::TyKind::Path(None, test_path("TestDescAndFn"))),
/* FP:test.rs-0290 */                     define_opaque: None,
/* FP:test.rs-0291 */                     // test::TestDescAndFn {
/* FP:test.rs-0292 */                     expr: Some(
/* FP:test.rs-0293 */                         cx.expr_struct(
/* FP:test.rs-0294 */                             sp,
/* FP:test.rs-0295 */                             test_path("TestDescAndFn"),
/* FP:test.rs-0296 */                             thin_vec![
/* FP:test.rs-0297 */                         // desc: test::TestDesc {
/* FP:test.rs-0298 */                         field(
/* FP:test.rs-0299 */                             "desc",
/* FP:test.rs-0300 */                             cx.expr_struct(sp, test_path("TestDesc"), thin_vec![
/* FP:test.rs-0301 */                                 // name: "path::to::test"
/* FP:test.rs-0302 */                                 field(
/* FP:test.rs-0303 */                                     "name",
/* FP:test.rs-0304 */                                     cx.expr_call(
/* FP:test.rs-0305 */                                         sp,
/* FP:test.rs-0306 */                                         cx.expr_path(test_path("StaticTestName")),
/* FP:test.rs-0307 */                                         thin_vec![cx.expr_str(sp, test_path_symbol)],
/* FP:test.rs-0308 */                                     ),
/* FP:test.rs-0309 */                                 ),
/* FP:test.rs-0310 */                                 // ignore: true | false
/* FP:test.rs-0311 */                                 field("ignore", cx.expr_bool(sp, should_ignore(&item)),),
/* FP:test.rs-0312 */                                 // ignore_message: Some("...") | None
/* FP:test.rs-0313 */                                 field(
/* FP:test.rs-0314 */                                     "ignore_message",
/* FP:test.rs-0315 */                                     if let Some(msg) = should_ignore_message(&item) {
/* FP:test.rs-0316 */                                         cx.expr_some(sp, cx.expr_str(sp, msg))
/* FP:test.rs-0317 */                                     } else {
/* FP:test.rs-0318 */                                         cx.expr_none(sp)
/* FP:test.rs-0319 */                                     },
/* FP:test.rs-0320 */                                 ),
/* FP:test.rs-0321 */                                 // source_file: <relative_path_of_source_file>
/* FP:test.rs-0322 */                                 field("source_file", cx.expr_str(sp, location_info.0)),
/* FP:test.rs-0323 */                                 // start_line: start line of the test fn identifier.
/* FP:test.rs-0324 */                                 field("start_line", cx.expr_usize(sp, location_info.1)),
/* FP:test.rs-0325 */                                 // start_col: start column of the test fn identifier.
/* FP:test.rs-0326 */                                 field("start_col", cx.expr_usize(sp, location_info.2)),
/* FP:test.rs-0327 */                                 // end_line: end line of the test fn identifier.
/* FP:test.rs-0328 */                                 field("end_line", cx.expr_usize(sp, location_info.3)),
/* FP:test.rs-0329 */                                 // end_col: end column of the test fn identifier.
/* FP:test.rs-0330 */                                 field("end_col", cx.expr_usize(sp, location_info.4)),
/* FP:test.rs-0331 */                                 // compile_fail: true | false
/* FP:test.rs-0332 */                                 field("compile_fail", cx.expr_bool(sp, false)),
/* FP:test.rs-0333 */                                 // no_run: true | false
/* FP:test.rs-0334 */                                 field("no_run", cx.expr_bool(sp, false)),
/* FP:test.rs-0335 */                                 // should_panic: ...
/* FP:test.rs-0336 */                                 field("should_panic", match should_panic(cx, &item) {
/* FP:test.rs-0337 */                                     // test::ShouldPanic::No
/* FP:test.rs-0338 */                                     ShouldPanic::No => {
/* FP:test.rs-0339 */                                         cx.expr_path(should_panic_path("No"))
/* FP:test.rs-0340 */                                     }
/* FP:test.rs-0341 */                                     // test::ShouldPanic::Yes
/* FP:test.rs-0342 */                                     ShouldPanic::Yes(None) => {
/* FP:test.rs-0343 */                                         cx.expr_path(should_panic_path("Yes"))
/* FP:test.rs-0344 */                                     }
/* FP:test.rs-0345 */                                     // test::ShouldPanic::YesWithMessage("...")
/* FP:test.rs-0346 */                                     ShouldPanic::Yes(Some(sym)) => cx.expr_call(
/* FP:test.rs-0347 */                                         sp,
/* FP:test.rs-0348 */                                         cx.expr_path(should_panic_path("YesWithMessage")),
/* FP:test.rs-0349 */                                         thin_vec![cx.expr_str(sp, sym)],
/* FP:test.rs-0350 */                                     ),
/* FP:test.rs-0351 */                                 },),
/* FP:test.rs-0352 */                                 // test_type: ...
/* FP:test.rs-0353 */                                 field("test_type", match test_type(cx) {
/* FP:test.rs-0354 */                                     // test::TestType::UnitTest
/* FP:test.rs-0355 */                                     TestType::UnitTest => {
/* FP:test.rs-0356 */                                         cx.expr_path(test_type_path("UnitTest"))
/* FP:test.rs-0357 */                                     }
/* FP:test.rs-0358 */                                     // test::TestType::IntegrationTest
/* FP:test.rs-0359 */                                     TestType::IntegrationTest => {
/* FP:test.rs-0360 */                                         cx.expr_path(test_type_path("IntegrationTest"))
/* FP:test.rs-0361 */                                     }
/* FP:test.rs-0362 */                                     // test::TestPath::Unknown
/* FP:test.rs-0363 */                                     TestType::Unknown => {
/* FP:test.rs-0364 */                                         cx.expr_path(test_type_path("Unknown"))
/* FP:test.rs-0365 */                                     }
/* FP:test.rs-0366 */                                 },),
/* FP:test.rs-0367 */                                 // },
/* FP:test.rs-0368 */                             ],),
/* FP:test.rs-0369 */                         ),
/* FP:test.rs-0370 */                         // testfn: test::StaticTestFn(...) | test::StaticBenchFn(...)
/* FP:test.rs-0371 */                         field("testfn", test_fn), // }
/* FP:test.rs-0372 */                     ],
/* FP:test.rs-0373 */                         ), // }
/* FP:test.rs-0374 */                     ),
/* FP:test.rs-0375 */                 }
/* FP:test.rs-0376 */                 .into(),
/* FP:test.rs-0377 */             ),
/* FP:test.rs-0378 */         );
/* FP:test.rs-0379 */     test_const.vis.kind = ast::VisibilityKind::Public;
/* FP:test.rs-0380 */ 
/* FP:test.rs-0381 */     // extern crate test
/* FP:test.rs-0382 */     let test_extern =
/* FP:test.rs-0383 */         cx.item(sp, ast::AttrVec::new(), ast::ItemKind::ExternCrate(None, test_ident));
/* FP:test.rs-0384 */ 
/* FP:test.rs-0385 */     debug!("synthetic test item:\n{}\n", pprust::item_to_string(&test_const));
/* FP:test.rs-0386 */ 
/* FP:test.rs-0387 */     if is_stmt {
/* FP:test.rs-0388 */         vec![
/* FP:test.rs-0389 */             // Access to libtest under a hygienic name
/* FP:test.rs-0390 */             Annotatable::Stmt(Box::new(cx.stmt_item(sp, test_extern))),
/* FP:test.rs-0391 */             // The generated test case
/* FP:test.rs-0392 */             Annotatable::Stmt(Box::new(cx.stmt_item(sp, test_const))),
/* FP:test.rs-0393 */             // The original item
/* FP:test.rs-0394 */             Annotatable::Stmt(Box::new(cx.stmt_item(sp, item))),
/* FP:test.rs-0395 */         ]
/* FP:test.rs-0396 */     } else {
/* FP:test.rs-0397 */         vec![
/* FP:test.rs-0398 */             // Access to libtest under a hygienic name
/* FP:test.rs-0399 */             Annotatable::Item(test_extern),
/* FP:test.rs-0400 */             // The generated test case
/* FP:test.rs-0401 */             Annotatable::Item(test_const),
/* FP:test.rs-0402 */             // The original item
/* FP:test.rs-0403 */             Annotatable::Item(item),
/* FP:test.rs-0404 */         ]
/* FP:test.rs-0405 */     }
/* FP:test.rs-0406 */ }
/* FP:test.rs-0407 */ 
/* FP:test.rs-0408 */ fn not_testable_error(cx: &ExtCtxt<'_>, attr_sp: Span, item: Option<&ast::Item>) {
/* FP:test.rs-0409 */     let dcx = cx.dcx();
/* FP:test.rs-0410 */     let msg = "the `#[test]` attribute may only be used on a non-associated function";
/* FP:test.rs-0411 */     let level = match item.map(|i| &i.kind) {
/* FP:test.rs-0412 */         // These were a warning before #92959 and need to continue being that to avoid breaking
/* FP:test.rs-0413 */         // stable user code (#94508).
/* FP:test.rs-0414 */         Some(ast::ItemKind::MacCall(_)) => Level::Warning,
/* FP:test.rs-0415 */         _ => Level::Error,
/* FP:test.rs-0416 */     };
/* FP:test.rs-0417 */     let mut err = Diag::<()>::new(dcx, level, msg);
/* FP:test.rs-0418 */     err.span(attr_sp);
/* FP:test.rs-0419 */     if let Some(item) = item {
/* FP:test.rs-0420 */         err.span_label(
/* FP:test.rs-0421 */             item.span,
/* FP:test.rs-0422 */             format!(
/* FP:test.rs-0423 */                 "expected a non-associated function, found {} {}",
/* FP:test.rs-0424 */                 item.kind.article(),
/* FP:test.rs-0425 */                 item.kind.descr()
/* FP:test.rs-0426 */             ),
/* FP:test.rs-0427 */         );
/* FP:test.rs-0428 */     }
/* FP:test.rs-0429 */     err.with_span_label(attr_sp, "the `#[test]` macro causes a function to be run as a test and has no effect on non-functions")
/* FP:test.rs-0430 */         .with_span_suggestion(attr_sp,
/* FP:test.rs-0431 */             "replace with conditional compilation to make the item only exist when tests are being run",
/* FP:test.rs-0432 */             "#[cfg(test)]",
/* FP:test.rs-0433 */             Applicability::MaybeIncorrect)
/* FP:test.rs-0434 */         .emit();
/* FP:test.rs-0435 */ }
/* FP:test.rs-0436 */ 
/* FP:test.rs-0437 */ fn get_location_info(cx: &ExtCtxt<'_>, fn_: &ast::Fn) -> (Symbol, usize, usize, usize, usize) {
/* FP:test.rs-0438 */     let span = fn_.ident.span;
/* FP:test.rs-0439 */     let (source_file, lo_line, lo_col, hi_line, hi_col) =
/* FP:test.rs-0440 */         cx.sess.source_map().span_to_location_info(span);
/* FP:test.rs-0441 */ 
/* FP:test.rs-0442 */     let file_name = match source_file {
/* FP:test.rs-0443 */         Some(sf) => sf.name.display(FileNameDisplayPreference::Remapped).to_string(),
/* FP:test.rs-0444 */         None => "no-location".to_string(),
/* FP:test.rs-0445 */     };
/* FP:test.rs-0446 */ 
/* FP:test.rs-0447 */     (Symbol::intern(&file_name), lo_line, lo_col, hi_line, hi_col)
/* FP:test.rs-0448 */ }
/* FP:test.rs-0449 */ 
/* FP:test.rs-0450 */ fn item_path(mod_path: &[Ident], item_ident: &Ident) -> String {
/* FP:test.rs-0451 */     join_path_idents(mod_path.iter().chain(iter::once(item_ident)))
/* FP:test.rs-0452 */ }
/* FP:test.rs-0453 */ 
/* FP:test.rs-0454 */ enum ShouldPanic {
/* FP:test.rs-0455 */     No,
/* FP:test.rs-0456 */     Yes(Option<Symbol>),
/* FP:test.rs-0457 */ }
/* FP:test.rs-0458 */ 
/* FP:test.rs-0459 */ fn should_ignore(i: &ast::Item) -> bool {
/* FP:test.rs-0460 */     attr::contains_name(&i.attrs, sym::ignore)
/* FP:test.rs-0461 */ }
/* FP:test.rs-0462 */ 
/* FP:test.rs-0463 */ fn should_ignore_message(i: &ast::Item) -> Option<Symbol> {
/* FP:test.rs-0464 */     match attr::find_by_name(&i.attrs, sym::ignore) {
/* FP:test.rs-0465 */         Some(attr) => {
/* FP:test.rs-0466 */             match attr.meta_item_list() {
/* FP:test.rs-0467 */                 // Handle #[ignore(bar = "foo")]
/* FP:test.rs-0468 */                 Some(_) => None,
/* FP:test.rs-0469 */                 // Handle #[ignore] and #[ignore = "message"]
/* FP:test.rs-0470 */                 None => attr.value_str(),
/* FP:test.rs-0471 */             }
/* FP:test.rs-0472 */         }
/* FP:test.rs-0473 */         None => None,
/* FP:test.rs-0474 */     }
/* FP:test.rs-0475 */ }
/* FP:test.rs-0476 */ 
/* FP:test.rs-0477 */ fn should_panic(cx: &ExtCtxt<'_>, i: &ast::Item) -> ShouldPanic {
/* FP:test.rs-0478 */     if let Some(Attribute::Parsed(AttributeKind::ShouldPanic { reason, .. })) =
/* FP:test.rs-0479 */         AttributeParser::parse_limited(
/* FP:test.rs-0480 */             cx.sess,
/* FP:test.rs-0481 */             &i.attrs,
/* FP:test.rs-0482 */             sym::should_panic,
/* FP:test.rs-0483 */             i.span,
/* FP:test.rs-0484 */             i.node_id(),
/* FP:test.rs-0485 */             None,
/* FP:test.rs-0486 */         )
/* FP:test.rs-0487 */     {
/* FP:test.rs-0488 */         ShouldPanic::Yes(reason)
/* FP:test.rs-0489 */     } else {
/* FP:test.rs-0490 */         ShouldPanic::No
/* FP:test.rs-0491 */     }
/* FP:test.rs-0492 */ }
/* FP:test.rs-0493 */ 
/* FP:test.rs-0494 */ enum TestType {
/* FP:test.rs-0495 */     UnitTest,
/* FP:test.rs-0496 */     IntegrationTest,
/* FP:test.rs-0497 */     Unknown,
/* FP:test.rs-0498 */ }
/* FP:test.rs-0499 */ 
/* FP:test.rs-0500 */ /// Attempts to determine the type of test.
/* FP:test.rs-0501 */ /// Since doctests are created without macro expanding, only possible variants here
/* FP:test.rs-0502 */ /// are `UnitTest`, `IntegrationTest` or `Unknown`.
/* FP:test.rs-0503 */ fn test_type(cx: &ExtCtxt<'_>) -> TestType {
/* FP:test.rs-0504 */     // Root path from context contains the topmost sources directory of the crate.
/* FP:test.rs-0505 */     // I.e., for `project` with sources in `src` and tests in `tests` folders
/* FP:test.rs-0506 */     // (no matter how many nested folders lie inside),
/* FP:test.rs-0507 */     // there will be two different root paths: `/project/src` and `/project/tests`.
/* FP:test.rs-0508 */     let crate_path = cx.root_path.as_path();
/* FP:test.rs-0509 */ 
/* FP:test.rs-0510 */     if crate_path.ends_with("src") {
/* FP:test.rs-0511 */         // `/src` folder contains unit-tests.
/* FP:test.rs-0512 */         TestType::UnitTest
/* FP:test.rs-0513 */     } else if crate_path.ends_with("tests") {
/* FP:test.rs-0514 */         // `/tests` folder contains integration tests.
/* FP:test.rs-0515 */         TestType::IntegrationTest
/* FP:test.rs-0516 */     } else {
/* FP:test.rs-0517 */         // Crate layout doesn't match expected one, test type is unknown.
/* FP:test.rs-0518 */         TestType::Unknown
/* FP:test.rs-0519 */     }
/* FP:test.rs-0520 */ }
/* FP:test.rs-0521 */ 
/* FP:test.rs-0522 */ fn check_test_signature(
/* FP:test.rs-0523 */     cx: &ExtCtxt<'_>,
/* FP:test.rs-0524 */     i: &ast::Item,
/* FP:test.rs-0525 */     f: &ast::Fn,
/* FP:test.rs-0526 */ ) -> Result<(), ErrorGuaranteed> {
/* FP:test.rs-0527 */     let has_should_panic_attr = attr::contains_name(&i.attrs, sym::should_panic);
/* FP:test.rs-0528 */     let dcx = cx.dcx();
/* FP:test.rs-0529 */ 
/* FP:test.rs-0530 */     if let ast::Safety::Unsafe(span) = f.sig.header.safety {
/* FP:test.rs-0531 */         return Err(dcx.emit_err(errors::TestBadFn { span: i.span, cause: span, kind: "unsafe" }));
/* FP:test.rs-0532 */     }
/* FP:test.rs-0533 */ 
/* FP:test.rs-0534 */     if let Some(coroutine_kind) = f.sig.header.coroutine_kind {
/* FP:test.rs-0535 */         match coroutine_kind {
/* FP:test.rs-0536 */             ast::CoroutineKind::Async { span, .. } => {
/* FP:test.rs-0537 */                 return Err(dcx.emit_err(errors::TestBadFn {
/* FP:test.rs-0538 */                     span: i.span,
/* FP:test.rs-0539 */                     cause: span,
/* FP:test.rs-0540 */                     kind: "async",
/* FP:test.rs-0541 */                 }));
/* FP:test.rs-0542 */             }
/* FP:test.rs-0543 */             ast::CoroutineKind::Gen { span, .. } => {
/* FP:test.rs-0544 */                 return Err(dcx.emit_err(errors::TestBadFn {
/* FP:test.rs-0545 */                     span: i.span,
/* FP:test.rs-0546 */                     cause: span,
/* FP:test.rs-0547 */                     kind: "gen",
/* FP:test.rs-0548 */                 }));
/* FP:test.rs-0549 */             }
/* FP:test.rs-0550 */             ast::CoroutineKind::AsyncGen { span, .. } => {
/* FP:test.rs-0551 */                 return Err(dcx.emit_err(errors::TestBadFn {
/* FP:test.rs-0552 */                     span: i.span,
/* FP:test.rs-0553 */                     cause: span,
/* FP:test.rs-0554 */                     kind: "async gen",
/* FP:test.rs-0555 */                 }));
/* FP:test.rs-0556 */             }
/* FP:test.rs-0557 */         }
/* FP:test.rs-0558 */     }
/* FP:test.rs-0559 */ 
/* FP:test.rs-0560 */     // If the termination trait is active, the compiler will check that the output
/* FP:test.rs-0561 */     // type implements the `Termination` trait as `libtest` enforces that.
/* FP:test.rs-0562 */     let has_output = match &f.sig.decl.output {
/* FP:test.rs-0563 */         ast::FnRetTy::Default(..) => false,
/* FP:test.rs-0564 */         ast::FnRetTy::Ty(t) if t.kind.is_unit() => false,
/* FP:test.rs-0565 */         _ => true,
/* FP:test.rs-0566 */     };
/* FP:test.rs-0567 */ 
/* FP:test.rs-0568 */     if !f.sig.decl.inputs.is_empty() {
/* FP:test.rs-0569 */         return Err(dcx.span_err(i.span, "functions used as tests can not have any arguments"));
/* FP:test.rs-0570 */     }
/* FP:test.rs-0571 */ 
/* FP:test.rs-0572 */     if has_should_panic_attr && has_output {
/* FP:test.rs-0573 */         return Err(dcx.span_err(i.span, "functions using `#[should_panic]` must return `()`"));
/* FP:test.rs-0574 */     }
/* FP:test.rs-0575 */ 
/* FP:test.rs-0576 */     if f.generics.params.iter().any(|param| !matches!(param.kind, GenericParamKind::Lifetime)) {
/* FP:test.rs-0577 */         return Err(dcx.span_err(
/* FP:test.rs-0578 */             i.span,
/* FP:test.rs-0579 */             "functions used as tests can not have any non-lifetime generic parameters",
/* FP:test.rs-0580 */         ));
/* FP:test.rs-0581 */     }
/* FP:test.rs-0582 */ 
/* FP:test.rs-0583 */     Ok(())
/* FP:test.rs-0584 */ }
/* FP:test.rs-0585 */ 
/* FP:test.rs-0586 */ fn check_bench_signature(
/* FP:test.rs-0587 */     cx: &ExtCtxt<'_>,
/* FP:test.rs-0588 */     i: &ast::Item,
/* FP:test.rs-0589 */     f: &ast::Fn,
/* FP:test.rs-0590 */ ) -> Result<(), ErrorGuaranteed> {
/* FP:test.rs-0591 */     // N.B., inadequate check, but we're running
/* FP:test.rs-0592 */     // well before resolve, can't get too deep.
/* FP:test.rs-0593 */     if f.sig.decl.inputs.len() != 1 {
/* FP:test.rs-0594 */         return Err(cx.dcx().emit_err(errors::BenchSig { span: i.span }));
/* FP:test.rs-0595 */     }
/* FP:test.rs-0596 */     Ok(())
/* FP:test.rs-0597 */ }