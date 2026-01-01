/* FP:nonstandard_style.rs-0001 */ use crate::rustc_abi::ExternAbi;
/* FP:nonstandard_style.rs-0002 */ use rustc_attr_parsing::AttributeParser;
/* FP:nonstandard_style.rs-0003 */ use crate::rustc_complete::Applicability;
/* FP:nonstandard_style.rs-0004 */ use crate::rustc_complete::attrs::{AttributeKind, ReprAttr};
/* FP:nonstandard_style.rs-0005 */ use crate::rustc_complete::def::{DefKind, Res};
/* FP:nonstandard_style.rs-0006 */ use crate::rustc_complete::def_id::DefId;
/* FP:nonstandard_style.rs-0007 */ use crate::rustc_complete::intravisit::{FnKind, Visitor};
/* FP:nonstandard_style.rs-0008 */ use crate::rustc_complete::{Attribute, GenericParamKind, PatExprKind, PatKind, find_attr};
/* FP:nonstandard_style.rs-0009 */ use crate::rustc_complete::hir::nested_filter::All;
/* FP:nonstandard_style.rs-0010 */ use crate::rustc_complete::ty::AssocContainer;
/* FP:nonstandard_style.rs-0011 */ use crate::rustc_complete::config::CrateType;
/* FP:nonstandard_style.rs-0012 */ use crate::rustc_complete::{declare_lint, declare_lint_pass};
/* FP:nonstandard_style.rs-0013 */ use crate::rustc_complete::def_id::LocalDefId;
/* FP:nonstandard_style.rs-0014 */ use crate::rustc_complete::{BytePos, Ident, Span, sym};
/* FP:nonstandard_style.rs-0015 */ use {rustc_ast as ast, rustc_hir as hir};
/* FP:nonstandard_style.rs-0016 */ 
/* FP:nonstandard_style.rs-0017 */ use crate::lints::{
/* FP:nonstandard_style.rs-0018 */     NonCamelCaseType, NonCamelCaseTypeSub, NonSnakeCaseDiag, NonSnakeCaseDiagSub,
/* FP:nonstandard_style.rs-0019 */     NonUpperCaseGlobal, NonUpperCaseGlobalSub, NonUpperCaseGlobalSubTool,
/* FP:nonstandard_style.rs-0020 */ };
/* FP:nonstandard_style.rs-0021 */ use crate::{EarlyContext, EarlyLintPass, LateContext, LateLintPass, LintContext};
/* FP:nonstandard_style.rs-0022 */ 
/* FP:nonstandard_style.rs-0023 */ declare_lint! {
/* FP:nonstandard_style.rs-0024 */     /// The `non_camel_case_types` lint detects types, variants, traits and
/* FP:nonstandard_style.rs-0025 */     /// type parameters that don't have camel case names.
/* FP:nonstandard_style.rs-0026 */     ///
/* FP:nonstandard_style.rs-0027 */     /// ### Example
/* FP:nonstandard_style.rs-0028 */     ///
/* FP:nonstandard_style.rs-0029 */     /// ```rust
/* FP:nonstandard_style.rs-0030 */     /// struct my_struct;
/* FP:nonstandard_style.rs-0031 */     /// ```
/* FP:nonstandard_style.rs-0032 */     ///
/* FP:nonstandard_style.rs-0033 */     /// {{produces}}
/* FP:nonstandard_style.rs-0034 */     ///
/* FP:nonstandard_style.rs-0035 */     /// ### Explanation
/* FP:nonstandard_style.rs-0036 */     ///
/* FP:nonstandard_style.rs-0037 */     /// The preferred style for these identifiers is to use "camel case", such
/* FP:nonstandard_style.rs-0038 */     /// as `MyStruct`, where the first letter should not be lowercase, and
/* FP:nonstandard_style.rs-0039 */     /// should not use underscores between letters. Underscores are allowed at
/* FP:nonstandard_style.rs-0040 */     /// the beginning and end of the identifier, as well as between
/* FP:nonstandard_style.rs-0041 */     /// non-letters (such as `X86_64`).
/* FP:nonstandard_style.rs-0042 */     pub NON_CAMEL_CASE_TYPES,
/* FP:nonstandard_style.rs-0043 */     Warn,
/* FP:nonstandard_style.rs-0044 */     "types, variants, traits and type parameters should have camel case names"
/* FP:nonstandard_style.rs-0045 */ }
/* FP:nonstandard_style.rs-0046 */ 
/* FP:nonstandard_style.rs-0047 */ declare_lint_pass!(NonCamelCaseTypes => [NON_CAMEL_CASE_TYPES]);
/* FP:nonstandard_style.rs-0048 */ 
/* FP:nonstandard_style.rs-0049 */ /// Some unicode characters *have* case, are considered upper case or lower case, but they *can't*
/* FP:nonstandard_style.rs-0050 */ /// be upper cased or lower cased. For the purposes of the lint suggestion, we care about being able
/* FP:nonstandard_style.rs-0051 */ /// to change the char's case.
/* FP:nonstandard_style.rs-0052 */ fn char_has_case(c: char) -> bool {
/* FP:nonstandard_style.rs-0053 */     let mut l = c.to_lowercase();
/* FP:nonstandard_style.rs-0054 */     let mut u = c.to_uppercase();
/* FP:nonstandard_style.rs-0055 */     while let Some(l) = l.next() {
/* FP:nonstandard_style.rs-0056 */         match u.next() {
/* FP:nonstandard_style.rs-0057 */             Some(u) if l != u => return true,
/* FP:nonstandard_style.rs-0058 */             _ => {}
/* FP:nonstandard_style.rs-0059 */         }
/* FP:nonstandard_style.rs-0060 */     }
/* FP:nonstandard_style.rs-0061 */     u.next().is_some()
/* FP:nonstandard_style.rs-0062 */ }
/* FP:nonstandard_style.rs-0063 */ 
/* FP:nonstandard_style.rs-0064 */ fn is_camel_case(name: &str) -> bool {
/* FP:nonstandard_style.rs-0065 */     let name = name.trim_matches('_');
/* FP:nonstandard_style.rs-0066 */     if name.is_empty() {
/* FP:nonstandard_style.rs-0067 */         return true;
/* FP:nonstandard_style.rs-0068 */     }
/* FP:nonstandard_style.rs-0069 */ 
/* FP:nonstandard_style.rs-0070 */     // start with a non-lowercase letter rather than non-uppercase
/* FP:nonstandard_style.rs-0071 */     // ones (some scripts don't have a concept of upper/lowercase)
/* FP:nonstandard_style.rs-0072 */     !name.chars().next().unwrap().is_lowercase()
/* FP:nonstandard_style.rs-0073 */         && !name.contains("__")
/* FP:nonstandard_style.rs-0074 */         && !name.chars().collect::<Vec<_>>().array_windows().any(|&[fst, snd]| {
/* FP:nonstandard_style.rs-0075 */             // contains a capitalisable character followed by, or preceded by, an underscore
/* FP:nonstandard_style.rs-0076 */             char_has_case(fst) && snd == '_' || char_has_case(snd) && fst == '_'
/* FP:nonstandard_style.rs-0077 */         })
/* FP:nonstandard_style.rs-0078 */ }
/* FP:nonstandard_style.rs-0079 */ 
/* FP:nonstandard_style.rs-0080 */ fn to_camel_case(s: &str) -> String {
/* FP:nonstandard_style.rs-0081 */     s.trim_matches('_')
/* FP:nonstandard_style.rs-0082 */         .split('_')
/* FP:nonstandard_style.rs-0083 */         .filter(|component| !component.is_empty())
/* FP:nonstandard_style.rs-0084 */         .map(|component| {
/* FP:nonstandard_style.rs-0085 */             let mut camel_cased_component = String::new();
/* FP:nonstandard_style.rs-0086 */ 
/* FP:nonstandard_style.rs-0087 */             let mut new_word = true;
/* FP:nonstandard_style.rs-0088 */             let mut prev_is_lower_case = true;
/* FP:nonstandard_style.rs-0089 */ 
/* FP:nonstandard_style.rs-0090 */             for c in component.chars() {
/* FP:nonstandard_style.rs-0091 */                 // Preserve the case if an uppercase letter follows a lowercase letter, so that
/* FP:nonstandard_style.rs-0092 */                 // `camelCase` is converted to `CamelCase`.
/* FP:nonstandard_style.rs-0093 */                 if prev_is_lower_case && c.is_uppercase() {
/* FP:nonstandard_style.rs-0094 */                     new_word = true;
/* FP:nonstandard_style.rs-0095 */                 }
/* FP:nonstandard_style.rs-0096 */ 
/* FP:nonstandard_style.rs-0097 */                 if new_word {
/* FP:nonstandard_style.rs-0098 */                     camel_cased_component.extend(c.to_uppercase());
/* FP:nonstandard_style.rs-0099 */                 } else {
/* FP:nonstandard_style.rs-0100 */                     camel_cased_component.extend(c.to_lowercase());
/* FP:nonstandard_style.rs-0101 */                 }
/* FP:nonstandard_style.rs-0102 */ 
/* FP:nonstandard_style.rs-0103 */                 prev_is_lower_case = c.is_lowercase();
/* FP:nonstandard_style.rs-0104 */                 new_word = false;
/* FP:nonstandard_style.rs-0105 */             }
/* FP:nonstandard_style.rs-0106 */ 
/* FP:nonstandard_style.rs-0107 */             camel_cased_component
/* FP:nonstandard_style.rs-0108 */         })
/* FP:nonstandard_style.rs-0109 */         .fold((String::new(), None), |(acc, prev): (String, Option<String>), next| {
/* FP:nonstandard_style.rs-0110 */             // separate two components with an underscore if their boundary cannot
/* FP:nonstandard_style.rs-0111 */             // be distinguished using an uppercase/lowercase case distinction
/* FP:nonstandard_style.rs-0112 */             let join = if let Some(prev) = prev {
/* FP:nonstandard_style.rs-0113 */                 let l = prev.chars().last().unwrap();
/* FP:nonstandard_style.rs-0114 */                 let f = next.chars().next().unwrap();
/* FP:nonstandard_style.rs-0115 */                 !char_has_case(l) && !char_has_case(f)
/* FP:nonstandard_style.rs-0116 */             } else {
/* FP:nonstandard_style.rs-0117 */                 false
/* FP:nonstandard_style.rs-0118 */             };
/* FP:nonstandard_style.rs-0119 */             (acc + if join { "_" } else { "" } + &next, Some(next))
/* FP:nonstandard_style.rs-0120 */         })
/* FP:nonstandard_style.rs-0121 */         .0
/* FP:nonstandard_style.rs-0122 */ }
/* FP:nonstandard_style.rs-0123 */ 
/* FP:nonstandard_style.rs-0124 */ impl NonCamelCaseTypes {
/* FP:nonstandard_style.rs-0125 */     fn check_case(&self, cx: &EarlyContext<'_>, sort: &str, ident: &Ident) {
/* FP:nonstandard_style.rs-0126 */         let name = ident.name.as_str();
/* FP:nonstandard_style.rs-0127 */ 
/* FP:nonstandard_style.rs-0128 */         if !is_camel_case(name) {
/* FP:nonstandard_style.rs-0129 */             let cc = to_camel_case(name);
/* FP:nonstandard_style.rs-0130 */             let sub = if *name != cc {
/* FP:nonstandard_style.rs-0131 */                 NonCamelCaseTypeSub::Suggestion { span: ident.span, replace: cc }
/* FP:nonstandard_style.rs-0132 */             } else {
/* FP:nonstandard_style.rs-0133 */                 NonCamelCaseTypeSub::Label { span: ident.span }
/* FP:nonstandard_style.rs-0134 */             };
/* FP:nonstandard_style.rs-0135 */             cx.emit_span_lint(
/* FP:nonstandard_style.rs-0136 */                 NON_CAMEL_CASE_TYPES,
/* FP:nonstandard_style.rs-0137 */                 ident.span,
/* FP:nonstandard_style.rs-0138 */                 NonCamelCaseType { sort, name, sub },
/* FP:nonstandard_style.rs-0139 */             );
/* FP:nonstandard_style.rs-0140 */         }
/* FP:nonstandard_style.rs-0141 */     }
/* FP:nonstandard_style.rs-0142 */ }
/* FP:nonstandard_style.rs-0143 */ 
/* FP:nonstandard_style.rs-0144 */ impl EarlyLintPass for NonCamelCaseTypes {
/* FP:nonstandard_style.rs-0145 */     fn check_item(&mut self, cx: &EarlyContext<'_>, it: &ast::Item) {
/* FP:nonstandard_style.rs-0146 */         let has_repr_c = matches!(
/* FP:nonstandard_style.rs-0147 */             AttributeParser::parse_limited(cx.sess(), &it.attrs, sym::repr, it.span, it.id, None),
/* FP:nonstandard_style.rs-0148 */             Some(Attribute::Parsed(AttributeKind::Repr { reprs, ..})) if reprs.iter().any(|(r, _)| r == &ReprAttr::ReprC)
/* FP:nonstandard_style.rs-0149 */         );
/* FP:nonstandard_style.rs-0150 */ 
/* FP:nonstandard_style.rs-0151 */         if has_repr_c {
/* FP:nonstandard_style.rs-0152 */             return;
/* FP:nonstandard_style.rs-0153 */         }
/* FP:nonstandard_style.rs-0154 */ 
/* FP:nonstandard_style.rs-0155 */         match &it.kind {
/* FP:nonstandard_style.rs-0156 */             ast::ItemKind::TyAlias(box ast::TyAlias { ident, .. })
/* FP:nonstandard_style.rs-0157 */             | ast::ItemKind::Enum(ident, ..)
/* FP:nonstandard_style.rs-0158 */             | ast::ItemKind::Struct(ident, ..)
/* FP:nonstandard_style.rs-0159 */             | ast::ItemKind::Union(ident, ..) => self.check_case(cx, "type", ident),
/* FP:nonstandard_style.rs-0160 */             ast::ItemKind::Trait(box ast::Trait { ident, .. }) => {
/* FP:nonstandard_style.rs-0161 */                 self.check_case(cx, "trait", ident)
/* FP:nonstandard_style.rs-0162 */             }
/* FP:nonstandard_style.rs-0163 */             ast::ItemKind::TraitAlias(ident, _, _) => self.check_case(cx, "trait alias", ident),
/* FP:nonstandard_style.rs-0164 */ 
/* FP:nonstandard_style.rs-0165 */             // N.B. This check is only for inherent associated types, so that we don't lint against
/* FP:nonstandard_style.rs-0166 */             // trait impls where we should have warned for the trait definition already.
/* FP:nonstandard_style.rs-0167 */             ast::ItemKind::Impl(ast::Impl { of_trait: None, items, .. }) => {
/* FP:nonstandard_style.rs-0168 */                 for it in items {
/* FP:nonstandard_style.rs-0169 */                     // FIXME: this doesn't respect `#[allow(..)]` on the item itself.
/* FP:nonstandard_style.rs-0170 */                     if let ast::AssocItemKind::Type(alias) = &it.kind {
/* FP:nonstandard_style.rs-0171 */                         self.check_case(cx, "associated type", &alias.ident);
/* FP:nonstandard_style.rs-0172 */                     }
/* FP:nonstandard_style.rs-0173 */                 }
/* FP:nonstandard_style.rs-0174 */             }
/* FP:nonstandard_style.rs-0175 */             _ => (),
/* FP:nonstandard_style.rs-0176 */         }
/* FP:nonstandard_style.rs-0177 */     }
/* FP:nonstandard_style.rs-0178 */ 
/* FP:nonstandard_style.rs-0179 */     fn check_trait_item(&mut self, cx: &EarlyContext<'_>, it: &ast::AssocItem) {
/* FP:nonstandard_style.rs-0180 */         if let ast::AssocItemKind::Type(alias) = &it.kind {
/* FP:nonstandard_style.rs-0181 */             self.check_case(cx, "associated type", &alias.ident);
/* FP:nonstandard_style.rs-0182 */         }
/* FP:nonstandard_style.rs-0183 */     }
/* FP:nonstandard_style.rs-0184 */ 
/* FP:nonstandard_style.rs-0185 */     fn check_variant(&mut self, cx: &EarlyContext<'_>, v: &ast::Variant) {
/* FP:nonstandard_style.rs-0186 */         self.check_case(cx, "variant", &v.ident);
/* FP:nonstandard_style.rs-0187 */     }
/* FP:nonstandard_style.rs-0188 */ 
/* FP:nonstandard_style.rs-0189 */     fn check_generic_param(&mut self, cx: &EarlyContext<'_>, param: &ast::GenericParam) {
/* FP:nonstandard_style.rs-0190 */         if let ast::GenericParamKind::Type { .. } = param.kind {
/* FP:nonstandard_style.rs-0191 */             self.check_case(cx, "type parameter", &param.ident);
/* FP:nonstandard_style.rs-0192 */         }
/* FP:nonstandard_style.rs-0193 */     }
/* FP:nonstandard_style.rs-0194 */ }
/* FP:nonstandard_style.rs-0195 */ 
/* FP:nonstandard_style.rs-0196 */ declare_lint! {
/* FP:nonstandard_style.rs-0197 */     /// The `non_snake_case` lint detects variables, methods, functions,
/* FP:nonstandard_style.rs-0198 */     /// lifetime parameters and modules that don't have snake case names.
/* FP:nonstandard_style.rs-0199 */     ///
/* FP:nonstandard_style.rs-0200 */     /// ### Example
/* FP:nonstandard_style.rs-0201 */     ///
/* FP:nonstandard_style.rs-0202 */     /// ```rust
/* FP:nonstandard_style.rs-0203 */     /// let MY_VALUE = 5;
/* FP:nonstandard_style.rs-0204 */     /// ```
/* FP:nonstandard_style.rs-0205 */     ///
/* FP:nonstandard_style.rs-0206 */     /// {{produces}}
/* FP:nonstandard_style.rs-0207 */     ///
/* FP:nonstandard_style.rs-0208 */     /// ### Explanation
/* FP:nonstandard_style.rs-0209 */     ///
/* FP:nonstandard_style.rs-0210 */     /// The preferred style for these identifiers is to use "snake case",
/* FP:nonstandard_style.rs-0211 */     /// where all the characters are in lowercase, with words separated with a
/* FP:nonstandard_style.rs-0212 */     /// single underscore, such as `my_value`.
/* FP:nonstandard_style.rs-0213 */     pub NON_SNAKE_CASE,
/* FP:nonstandard_style.rs-0214 */     Warn,
/* FP:nonstandard_style.rs-0215 */     "variables, methods, functions, lifetime parameters and modules should have snake case names"
/* FP:nonstandard_style.rs-0216 */ }
/* FP:nonstandard_style.rs-0217 */ 
/* FP:nonstandard_style.rs-0218 */ declare_lint_pass!(NonSnakeCase => [NON_SNAKE_CASE]);
/* FP:nonstandard_style.rs-0219 */ 
/* FP:nonstandard_style.rs-0220 */ impl NonSnakeCase {
/* FP:nonstandard_style.rs-0221 */     fn to_snake_case(mut name: &str) -> String {
/* FP:nonstandard_style.rs-0222 */         let mut words = vec![];
/* FP:nonstandard_style.rs-0223 */         // Preserve leading underscores
/* FP:nonstandard_style.rs-0224 */         name = name.trim_start_matches(|c: char| {
/* FP:nonstandard_style.rs-0225 */             if c == '_' {
/* FP:nonstandard_style.rs-0226 */                 words.push(String::new());
/* FP:nonstandard_style.rs-0227 */                 true
/* FP:nonstandard_style.rs-0228 */             } else {
/* FP:nonstandard_style.rs-0229 */                 false
/* FP:nonstandard_style.rs-0230 */             }
/* FP:nonstandard_style.rs-0231 */         });
/* FP:nonstandard_style.rs-0232 */         for s in name.split('_') {
/* FP:nonstandard_style.rs-0233 */             let mut last_upper = false;
/* FP:nonstandard_style.rs-0234 */             let mut buf = String::new();
/* FP:nonstandard_style.rs-0235 */             if s.is_empty() {
/* FP:nonstandard_style.rs-0236 */                 continue;
/* FP:nonstandard_style.rs-0237 */             }
/* FP:nonstandard_style.rs-0238 */             for ch in s.chars() {
/* FP:nonstandard_style.rs-0239 */                 if !buf.is_empty() && buf != "'" && ch.is_uppercase() && !last_upper {
/* FP:nonstandard_style.rs-0240 */                     words.push(buf);
/* FP:nonstandard_style.rs-0241 */                     buf = String::new();
/* FP:nonstandard_style.rs-0242 */                 }
/* FP:nonstandard_style.rs-0243 */                 last_upper = ch.is_uppercase();
/* FP:nonstandard_style.rs-0244 */                 buf.extend(ch.to_lowercase());
/* FP:nonstandard_style.rs-0245 */             }
/* FP:nonstandard_style.rs-0246 */             words.push(buf);
/* FP:nonstandard_style.rs-0247 */         }
/* FP:nonstandard_style.rs-0248 */         words.join("_")
/* FP:nonstandard_style.rs-0249 */     }
/* FP:nonstandard_style.rs-0250 */ 
/* FP:nonstandard_style.rs-0251 */     /// Checks if a given identifier is snake case, and reports a diagnostic if not.
/* FP:nonstandard_style.rs-0252 */     fn check_snake_case(&self, cx: &LateContext<'_>, sort: &str, ident: &Ident) {
/* FP:nonstandard_style.rs-0253 */         fn is_snake_case(ident: &str) -> bool {
/* FP:nonstandard_style.rs-0254 */             if ident.is_empty() {
/* FP:nonstandard_style.rs-0255 */                 return true;
/* FP:nonstandard_style.rs-0256 */             }
/* FP:nonstandard_style.rs-0257 */             let ident = ident.trim_start_matches('\'');
/* FP:nonstandard_style.rs-0258 */             let ident = ident.trim_matches('_');
/* FP:nonstandard_style.rs-0259 */ 
/* FP:nonstandard_style.rs-0260 */             if ident.contains("__") {
/* FP:nonstandard_style.rs-0261 */                 return false;
/* FP:nonstandard_style.rs-0262 */             }
/* FP:nonstandard_style.rs-0263 */ 
/* FP:nonstandard_style.rs-0264 */             // This correctly handles letters in languages with and without
/* FP:nonstandard_style.rs-0265 */             // cases, as well as numbers and underscores.
/* FP:nonstandard_style.rs-0266 */             !ident.chars().any(char::is_uppercase)
/* FP:nonstandard_style.rs-0267 */         }
/* FP:nonstandard_style.rs-0268 */ 
/* FP:nonstandard_style.rs-0269 */         let name = ident.name.as_str();
/* FP:nonstandard_style.rs-0270 */ 
/* FP:nonstandard_style.rs-0271 */         if !is_snake_case(name) {
/* FP:nonstandard_style.rs-0272 */             let span = ident.span;
/* FP:nonstandard_style.rs-0273 */             let sc = NonSnakeCase::to_snake_case(name);
/* FP:nonstandard_style.rs-0274 */             // We cannot provide meaningful suggestions
/* FP:nonstandard_style.rs-0275 */             // if the characters are in the category of "Uppercase Letter".
/* FP:nonstandard_style.rs-0276 */             let sub = if name != sc {
/* FP:nonstandard_style.rs-0277 */                 // We have a valid span in almost all cases, but we don't have one when linting a
/* FP:nonstandard_style.rs-0278 */                 // crate name provided via the command line.
/* FP:nonstandard_style.rs-0279 */                 if !span.is_dummy() {
/* FP:nonstandard_style.rs-0280 */                     let sc_ident = Ident::from_str_and_span(&sc, span);
/* FP:nonstandard_style.rs-0281 */                     if sc_ident.is_reserved() {
/* FP:nonstandard_style.rs-0282 */                         // We shouldn't suggest a reserved identifier to fix non-snake-case
/* FP:nonstandard_style.rs-0283 */                         // identifiers. Instead, recommend renaming the identifier entirely or, if
/* FP:nonstandard_style.rs-0284 */                         // permitted, escaping it to create a raw identifier.
/* FP:nonstandard_style.rs-0285 */                         if sc_ident.name.can_be_raw() {
/* FP:nonstandard_style.rs-0286 */                             NonSnakeCaseDiagSub::RenameOrConvertSuggestion {
/* FP:nonstandard_style.rs-0287 */                                 span,
/* FP:nonstandard_style.rs-0288 */                                 suggestion: sc_ident,
/* FP:nonstandard_style.rs-0289 */                             }
/* FP:nonstandard_style.rs-0290 */                         } else {
/* FP:nonstandard_style.rs-0291 */                             NonSnakeCaseDiagSub::SuggestionAndNote { span }
/* FP:nonstandard_style.rs-0292 */                         }
/* FP:nonstandard_style.rs-0293 */                     } else {
/* FP:nonstandard_style.rs-0294 */                         NonSnakeCaseDiagSub::ConvertSuggestion { span, suggestion: sc.clone() }
/* FP:nonstandard_style.rs-0295 */                     }
/* FP:nonstandard_style.rs-0296 */                 } else {
/* FP:nonstandard_style.rs-0297 */                     NonSnakeCaseDiagSub::Help
/* FP:nonstandard_style.rs-0298 */                 }
/* FP:nonstandard_style.rs-0299 */             } else {
/* FP:nonstandard_style.rs-0300 */                 NonSnakeCaseDiagSub::Label { span }
/* FP:nonstandard_style.rs-0301 */             };
/* FP:nonstandard_style.rs-0302 */             cx.emit_span_lint(NON_SNAKE_CASE, span, NonSnakeCaseDiag { sort, name, sc, sub });
/* FP:nonstandard_style.rs-0303 */         }
/* FP:nonstandard_style.rs-0304 */     }
/* FP:nonstandard_style.rs-0305 */ }
/* FP:nonstandard_style.rs-0306 */ 
/* FP:nonstandard_style.rs-0307 */ impl<'tcx> LateLintPass<'tcx> for NonSnakeCase {
/* FP:nonstandard_style.rs-0308 */     fn check_mod(&mut self, cx: &LateContext<'_>, _: &'tcx hir::Mod<'tcx>, id: hir::HirId) {
/* FP:nonstandard_style.rs-0309 */         if id != hir::CRATE_HIR_ID {
/* FP:nonstandard_style.rs-0310 */             return;
/* FP:nonstandard_style.rs-0311 */         }
/* FP:nonstandard_style.rs-0312 */ 
/* FP:nonstandard_style.rs-0313 */         // Issue #45127: don't enforce `snake_case` for binary crates as binaries are not intended
/* FP:nonstandard_style.rs-0314 */         // to be distributed and depended on like libraries. The lint is not suppressed for cdylib
/* FP:nonstandard_style.rs-0315 */         // or staticlib because it's not clear what the desired lint behavior for those are.
/* FP:nonstandard_style.rs-0316 */         if cx.tcx.crate_types().iter().all(|&crate_type| crate_type == CrateType::Executable) {
/* FP:nonstandard_style.rs-0317 */             return;
/* FP:nonstandard_style.rs-0318 */         }
/* FP:nonstandard_style.rs-0319 */ 
/* FP:nonstandard_style.rs-0320 */         let crate_ident = if let Some(name) = &cx.tcx.sess.opts.crate_name {
/* FP:nonstandard_style.rs-0321 */             Some(Ident::from_str(name))
/* FP:nonstandard_style.rs-0322 */         } else {
/* FP:nonstandard_style.rs-0323 */             find_attr!(cx.tcx.hir_attrs(hir::CRATE_HIR_ID), AttributeKind::CrateName{name, name_span,..} => (name, name_span)).map(
/* FP:nonstandard_style.rs-0324 */                 |(&name, &span)| {
/* FP:nonstandard_style.rs-0325 */                     // Discard the double quotes surrounding the literal.
/* FP:nonstandard_style.rs-0326 */                     let sp = cx
/* FP:nonstandard_style.rs-0327 */                         .sess()
/* FP:nonstandard_style.rs-0328 */                         .source_map()
/* FP:nonstandard_style.rs-0329 */                         .span_to_snippet(span)
/* FP:nonstandard_style.rs-0330 */                         .ok()
/* FP:nonstandard_style.rs-0331 */                         .and_then(|snippet| {
/* FP:nonstandard_style.rs-0332 */                             let left = snippet.find('"')?;
/* FP:nonstandard_style.rs-0333 */                             let right = snippet.rfind('"').map(|pos| snippet.len() - pos)?;
/* FP:nonstandard_style.rs-0334 */ 
/* FP:nonstandard_style.rs-0335 */                             Some(
/* FP:nonstandard_style.rs-0336 */                                 span
/* FP:nonstandard_style.rs-0337 */                                     .with_lo(span.lo() + BytePos(left as u32 + 1))
/* FP:nonstandard_style.rs-0338 */                                     .with_hi(span.hi() - BytePos(right as u32)),
/* FP:nonstandard_style.rs-0339 */                             )
/* FP:nonstandard_style.rs-0340 */                         })
/* FP:nonstandard_style.rs-0341 */                         .unwrap_or(span);
/* FP:nonstandard_style.rs-0342 */ 
/* FP:nonstandard_style.rs-0343 */                     Ident::new(name, sp)
/* FP:nonstandard_style.rs-0344 */                 },
/* FP:nonstandard_style.rs-0345 */             )
/* FP:nonstandard_style.rs-0346 */         };
/* FP:nonstandard_style.rs-0347 */ 
/* FP:nonstandard_style.rs-0348 */         if let Some(ident) = &crate_ident {
/* FP:nonstandard_style.rs-0349 */             self.check_snake_case(cx, "crate", ident);
/* FP:nonstandard_style.rs-0350 */         }
/* FP:nonstandard_style.rs-0351 */     }
/* FP:nonstandard_style.rs-0352 */ 
/* FP:nonstandard_style.rs-0353 */     fn check_generic_param(&mut self, cx: &LateContext<'_>, param: &hir::GenericParam<'_>) {
/* FP:nonstandard_style.rs-0354 */         if let GenericParamKind::Lifetime { .. } = param.kind {
/* FP:nonstandard_style.rs-0355 */             self.check_snake_case(cx, "lifetime", &param.name.ident());
/* FP:nonstandard_style.rs-0356 */         }
/* FP:nonstandard_style.rs-0357 */     }
/* FP:nonstandard_style.rs-0358 */ 
/* FP:nonstandard_style.rs-0359 */     fn check_fn(
/* FP:nonstandard_style.rs-0360 */         &mut self,
/* FP:nonstandard_style.rs-0361 */         cx: &LateContext<'_>,
/* FP:nonstandard_style.rs-0362 */         fk: FnKind<'_>,
/* FP:nonstandard_style.rs-0363 */         _: &hir::FnDecl<'_>,
/* FP:nonstandard_style.rs-0364 */         _: &hir::Body<'_>,
/* FP:nonstandard_style.rs-0365 */         _: Span,
/* FP:nonstandard_style.rs-0366 */         id: LocalDefId,
/* FP:nonstandard_style.rs-0367 */     ) {
/* FP:nonstandard_style.rs-0368 */         match &fk {
/* FP:nonstandard_style.rs-0369 */             FnKind::Method(ident, sig, ..) => match cx.tcx.associated_item(id).container {
/* FP:nonstandard_style.rs-0370 */                 AssocContainer::InherentImpl => {
/* FP:nonstandard_style.rs-0371 */                     if sig.header.abi != ExternAbi::Rust
/* FP:nonstandard_style.rs-0372 */                         && find_attr!(cx.tcx.get_all_attrs(id), AttributeKind::NoMangle(..))
/* FP:nonstandard_style.rs-0373 */                     {
/* FP:nonstandard_style.rs-0374 */                         return;
/* FP:nonstandard_style.rs-0375 */                     }
/* FP:nonstandard_style.rs-0376 */                     self.check_snake_case(cx, "method", ident);
/* FP:nonstandard_style.rs-0377 */                 }
/* FP:nonstandard_style.rs-0378 */                 AssocContainer::Trait => {
/* FP:nonstandard_style.rs-0379 */                     self.check_snake_case(cx, "trait method", ident);
/* FP:nonstandard_style.rs-0380 */                 }
/* FP:nonstandard_style.rs-0381 */                 AssocContainer::TraitImpl(_) => {}
/* FP:nonstandard_style.rs-0382 */             },
/* FP:nonstandard_style.rs-0383 */             FnKind::ItemFn(ident, _, header) => {
/* FP:nonstandard_style.rs-0384 */                 // Skip foreign-ABI #[unsafe(no_mangle)] functions (Issue #31924)
/* FP:nonstandard_style.rs-0385 */                 if header.abi != ExternAbi::Rust
/* FP:nonstandard_style.rs-0386 */                     && find_attr!(cx.tcx.get_all_attrs(id), AttributeKind::NoMangle(..))
/* FP:nonstandard_style.rs-0387 */                 {
/* FP:nonstandard_style.rs-0388 */                     return;
/* FP:nonstandard_style.rs-0389 */                 }
/* FP:nonstandard_style.rs-0390 */                 self.check_snake_case(cx, "function", ident);
/* FP:nonstandard_style.rs-0391 */             }
/* FP:nonstandard_style.rs-0392 */             FnKind::Closure => (),
/* FP:nonstandard_style.rs-0393 */         }
/* FP:nonstandard_style.rs-0394 */     }
/* FP:nonstandard_style.rs-0395 */ 
/* FP:nonstandard_style.rs-0396 */     fn check_item(&mut self, cx: &LateContext<'_>, it: &hir::Item<'_>) {
/* FP:nonstandard_style.rs-0397 */         if let hir::ItemKind::Mod(ident, _) = it.kind {
/* FP:nonstandard_style.rs-0398 */             self.check_snake_case(cx, "module", &ident);
/* FP:nonstandard_style.rs-0399 */         }
/* FP:nonstandard_style.rs-0400 */     }
/* FP:nonstandard_style.rs-0401 */ 
/* FP:nonstandard_style.rs-0402 */     fn check_ty(&mut self, cx: &LateContext<'_>, ty: &hir::Ty<'_, hir::AmbigArg>) {
/* FP:nonstandard_style.rs-0403 */         if let hir::TyKind::FnPtr(hir::FnPtrTy { param_idents, .. }) = &ty.kind {
/* FP:nonstandard_style.rs-0404 */             for param_ident in *param_idents {
/* FP:nonstandard_style.rs-0405 */                 if let Some(param_ident) = param_ident {
/* FP:nonstandard_style.rs-0406 */                     self.check_snake_case(cx, "variable", param_ident);
/* FP:nonstandard_style.rs-0407 */                 }
/* FP:nonstandard_style.rs-0408 */             }
/* FP:nonstandard_style.rs-0409 */         }
/* FP:nonstandard_style.rs-0410 */     }
/* FP:nonstandard_style.rs-0411 */ 
/* FP:nonstandard_style.rs-0412 */     fn check_trait_item(&mut self, cx: &LateContext<'_>, item: &hir::TraitItem<'_>) {
/* FP:nonstandard_style.rs-0413 */         if let hir::TraitItemKind::Fn(_, hir::TraitFn::Required(param_idents)) = item.kind {
/* FP:nonstandard_style.rs-0414 */             self.check_snake_case(cx, "trait method", &item.ident);
/* FP:nonstandard_style.rs-0415 */             for param_ident in param_idents {
/* FP:nonstandard_style.rs-0416 */                 if let Some(param_ident) = param_ident {
/* FP:nonstandard_style.rs-0417 */                     self.check_snake_case(cx, "variable", param_ident);
/* FP:nonstandard_style.rs-0418 */                 }
/* FP:nonstandard_style.rs-0419 */             }
/* FP:nonstandard_style.rs-0420 */         }
/* FP:nonstandard_style.rs-0421 */     }
/* FP:nonstandard_style.rs-0422 */ 
/* FP:nonstandard_style.rs-0423 */     fn check_pat(&mut self, cx: &LateContext<'_>, p: &hir::Pat<'_>) {
/* FP:nonstandard_style.rs-0424 */         if let PatKind::Binding(_, hid, ident, _) = p.kind {
/* FP:nonstandard_style.rs-0425 */             if let hir::Node::PatField(field) = cx.tcx.parent_hir_node(hid) {
/* FP:nonstandard_style.rs-0426 */                 if !field.is_shorthand {
/* FP:nonstandard_style.rs-0427 */                     // Only check if a new name has been introduced, to avoid warning
/* FP:nonstandard_style.rs-0428 */                     // on both the struct definition and this pattern.
/* FP:nonstandard_style.rs-0429 */                     self.check_snake_case(cx, "variable", &ident);
/* FP:nonstandard_style.rs-0430 */                 }
/* FP:nonstandard_style.rs-0431 */                 return;
/* FP:nonstandard_style.rs-0432 */             }
/* FP:nonstandard_style.rs-0433 */             self.check_snake_case(cx, "variable", &ident);
/* FP:nonstandard_style.rs-0434 */         }
/* FP:nonstandard_style.rs-0435 */     }
/* FP:nonstandard_style.rs-0436 */ 
/* FP:nonstandard_style.rs-0437 */     fn check_struct_def(&mut self, cx: &LateContext<'_>, s: &hir::VariantData<'_>) {
/* FP:nonstandard_style.rs-0438 */         for sf in s.fields() {
/* FP:nonstandard_style.rs-0439 */             self.check_snake_case(cx, "structure field", &sf.ident);
/* FP:nonstandard_style.rs-0440 */         }
/* FP:nonstandard_style.rs-0441 */     }
/* FP:nonstandard_style.rs-0442 */ }
/* FP:nonstandard_style.rs-0443 */ 
/* FP:nonstandard_style.rs-0444 */ declare_lint! {
/* FP:nonstandard_style.rs-0445 */     /// The `non_upper_case_globals` lint detects static items that don't have
/* FP:nonstandard_style.rs-0446 */     /// uppercase identifiers.
/* FP:nonstandard_style.rs-0447 */     ///
/* FP:nonstandard_style.rs-0448 */     /// ### Example
/* FP:nonstandard_style.rs-0449 */     ///
/* FP:nonstandard_style.rs-0450 */     /// ```rust
/* FP:nonstandard_style.rs-0451 */     /// static max_points: i32 = 5;
/* FP:nonstandard_style.rs-0452 */     /// ```
/* FP:nonstandard_style.rs-0453 */     ///
/* FP:nonstandard_style.rs-0454 */     /// {{produces}}
/* FP:nonstandard_style.rs-0455 */     ///
/* FP:nonstandard_style.rs-0456 */     /// ### Explanation
/* FP:nonstandard_style.rs-0457 */     ///
/* FP:nonstandard_style.rs-0458 */     /// The preferred style is for static item names to use all uppercase
/* FP:nonstandard_style.rs-0459 */     /// letters such as `MAX_POINTS`.
/* FP:nonstandard_style.rs-0460 */     pub NON_UPPER_CASE_GLOBALS,
/* FP:nonstandard_style.rs-0461 */     Warn,
/* FP:nonstandard_style.rs-0462 */     "static constants should have uppercase identifiers"
/* FP:nonstandard_style.rs-0463 */ }
/* FP:nonstandard_style.rs-0464 */ 
/* FP:nonstandard_style.rs-0465 */ declare_lint_pass!(NonUpperCaseGlobals => [NON_UPPER_CASE_GLOBALS]);
/* FP:nonstandard_style.rs-0466 */ 
/* FP:nonstandard_style.rs-0467 */ impl NonUpperCaseGlobals {
/* FP:nonstandard_style.rs-0468 */     fn check_upper_case(cx: &LateContext<'_>, sort: &str, did: Option<LocalDefId>, ident: &Ident) {
/* FP:nonstandard_style.rs-0469 */         let name = ident.name.as_str();
/* FP:nonstandard_style.rs-0470 */         if name.chars().any(|c| c.is_lowercase()) {
/* FP:nonstandard_style.rs-0471 */             let uc = NonSnakeCase::to_snake_case(name).to_uppercase();
/* FP:nonstandard_style.rs-0472 */ 
/* FP:nonstandard_style.rs-0473 */             // If the item is exported, suggesting changing it's name would be breaking-change
/* FP:nonstandard_style.rs-0474 */             // and could break users without a "nice" applicable fix, so let's avoid it.
/* FP:nonstandard_style.rs-0475 */             let can_change_usages = if let Some(did) = did {
/* FP:nonstandard_style.rs-0476 */                 !cx.tcx.effective_visibilities(()).is_exported(did)
/* FP:nonstandard_style.rs-0477 */             } else {
/* FP:nonstandard_style.rs-0478 */                 false
/* FP:nonstandard_style.rs-0479 */             };
/* FP:nonstandard_style.rs-0480 */ 
/* FP:nonstandard_style.rs-0481 */             // We cannot provide meaningful suggestions
/* FP:nonstandard_style.rs-0482 */             // if the characters are in the category of "Lowercase Letter".
/* FP:nonstandard_style.rs-0483 */             let sub = if *name != uc {
/* FP:nonstandard_style.rs-0484 */                 NonUpperCaseGlobalSub::Suggestion {
/* FP:nonstandard_style.rs-0485 */                     span: ident.span,
/* FP:nonstandard_style.rs-0486 */                     replace: uc.clone(),
/* FP:nonstandard_style.rs-0487 */                     applicability: if can_change_usages {
/* FP:nonstandard_style.rs-0488 */                         Applicability::MachineApplicable
/* FP:nonstandard_style.rs-0489 */                     } else {
/* FP:nonstandard_style.rs-0490 */                         Applicability::MaybeIncorrect
/* FP:nonstandard_style.rs-0491 */                     },
/* FP:nonstandard_style.rs-0492 */                 }
/* FP:nonstandard_style.rs-0493 */             } else {
/* FP:nonstandard_style.rs-0494 */                 NonUpperCaseGlobalSub::Label { span: ident.span }
/* FP:nonstandard_style.rs-0495 */             };
/* FP:nonstandard_style.rs-0496 */ 
/* FP:nonstandard_style.rs-0497 */             struct UsageCollector<'a, 'tcx> {
/* FP:nonstandard_style.rs-0498 */                 cx: &'tcx LateContext<'a>,
/* FP:nonstandard_style.rs-0499 */                 did: DefId,
/* FP:nonstandard_style.rs-0500 */                 collected: Vec<Span>,
/* FP:nonstandard_style.rs-0501 */             }
/* FP:nonstandard_style.rs-0502 */ 
/* FP:nonstandard_style.rs-0503 */             impl<'v, 'tcx> Visitor<'v> for UsageCollector<'v, 'tcx> {
/* FP:nonstandard_style.rs-0504 */                 type NestedFilter = All;
/* FP:nonstandard_style.rs-0505 */ 
/* FP:nonstandard_style.rs-0506 */                 fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
/* FP:nonstandard_style.rs-0507 */                     self.cx.tcx
/* FP:nonstandard_style.rs-0508 */                 }
/* FP:nonstandard_style.rs-0509 */ 
/* FP:nonstandard_style.rs-0510 */                 fn visit_path(
/* FP:nonstandard_style.rs-0511 */                     &mut self,
/* FP:nonstandard_style.rs-0512 */                     path: &crate::rustc_hir::Path<'v>,
/* FP:nonstandard_style.rs-0513 */                     _id: crate::rustc_hir::HirId,
/* FP:nonstandard_style.rs-0514 */                 ) -> Self::Result {
/* FP:nonstandard_style.rs-0515 */                     if let Some(final_seg) = path.segments.last()
/* FP:nonstandard_style.rs-0516 */                         && final_seg.res.opt_def_id() == Some(self.did)
/* FP:nonstandard_style.rs-0517 */                     {
/* FP:nonstandard_style.rs-0518 */                         self.collected.push(final_seg.ident.span);
/* FP:nonstandard_style.rs-0519 */                     }
/* FP:nonstandard_style.rs-0520 */                 }
/* FP:nonstandard_style.rs-0521 */             }
/* FP:nonstandard_style.rs-0522 */ 
/* FP:nonstandard_style.rs-0523 */             cx.emit_span_lint_lazy(NON_UPPER_CASE_GLOBALS, ident.span, || {
/* FP:nonstandard_style.rs-0524 */                 // Compute usages lazily as it can expansive and useless when the lint is allowed.
/* FP:nonstandard_style.rs-0525 */                 // cf. https://github.com/rust-lang/rust/pull/142645#issuecomment-2993024625
/* FP:nonstandard_style.rs-0526 */                 let usages = if can_change_usages
/* FP:nonstandard_style.rs-0527 */                     && *name != uc
/* FP:nonstandard_style.rs-0528 */                     && let Some(did) = did
/* FP:nonstandard_style.rs-0529 */                 {
/* FP:nonstandard_style.rs-0530 */                     let mut usage_collector =
/* FP:nonstandard_style.rs-0531 */                         UsageCollector { cx, did: did.to_def_id(), collected: Vec::new() };
/* FP:nonstandard_style.rs-0532 */                     cx.tcx.hir_walk_toplevel_module(&mut usage_collector);
/* FP:nonstandard_style.rs-0533 */                     usage_collector
/* FP:nonstandard_style.rs-0534 */                         .collected
/* FP:nonstandard_style.rs-0535 */                         .into_iter()
/* FP:nonstandard_style.rs-0536 */                         .map(|span| NonUpperCaseGlobalSubTool { span, replace: uc.clone() })
/* FP:nonstandard_style.rs-0537 */                         .collect()
/* FP:nonstandard_style.rs-0538 */                 } else {
/* FP:nonstandard_style.rs-0539 */                     vec![]
/* FP:nonstandard_style.rs-0540 */                 };
/* FP:nonstandard_style.rs-0541 */ 
/* FP:nonstandard_style.rs-0542 */                 NonUpperCaseGlobal { sort, name, sub, usages }
/* FP:nonstandard_style.rs-0543 */             });
/* FP:nonstandard_style.rs-0544 */         }
/* FP:nonstandard_style.rs-0545 */     }
/* FP:nonstandard_style.rs-0546 */ }
/* FP:nonstandard_style.rs-0547 */ 
/* FP:nonstandard_style.rs-0548 */ impl<'tcx> LateLintPass<'tcx> for NonUpperCaseGlobals {
/* FP:nonstandard_style.rs-0549 */     fn check_item(&mut self, cx: &LateContext<'_>, it: &hir::Item<'_>) {
/* FP:nonstandard_style.rs-0550 */         let attrs = cx.tcx.hir_attrs(it.hir_id());
/* FP:nonstandard_style.rs-0551 */         match it.kind {
/* FP:nonstandard_style.rs-0552 */             hir::ItemKind::Static(_, ident, ..)
/* FP:nonstandard_style.rs-0553 */                 if !find_attr!(attrs, AttributeKind::NoMangle(..)) =>
/* FP:nonstandard_style.rs-0554 */             {
/* FP:nonstandard_style.rs-0555 */                 NonUpperCaseGlobals::check_upper_case(
/* FP:nonstandard_style.rs-0556 */                     cx,
/* FP:nonstandard_style.rs-0557 */                     "static variable",
/* FP:nonstandard_style.rs-0558 */                     Some(it.owner_id.def_id),
/* FP:nonstandard_style.rs-0559 */                     &ident,
/* FP:nonstandard_style.rs-0560 */                 );
/* FP:nonstandard_style.rs-0561 */             }
/* FP:nonstandard_style.rs-0562 */             hir::ItemKind::Const(ident, ..) => {
/* FP:nonstandard_style.rs-0563 */                 NonUpperCaseGlobals::check_upper_case(
/* FP:nonstandard_style.rs-0564 */                     cx,
/* FP:nonstandard_style.rs-0565 */                     "constant",
/* FP:nonstandard_style.rs-0566 */                     Some(it.owner_id.def_id),
/* FP:nonstandard_style.rs-0567 */                     &ident,
/* FP:nonstandard_style.rs-0568 */                 );
/* FP:nonstandard_style.rs-0569 */             }
/* FP:nonstandard_style.rs-0570 */             _ => {}
/* FP:nonstandard_style.rs-0571 */         }
/* FP:nonstandard_style.rs-0572 */     }
/* FP:nonstandard_style.rs-0573 */ 
/* FP:nonstandard_style.rs-0574 */     fn check_trait_item(&mut self, cx: &LateContext<'_>, ti: &hir::TraitItem<'_>) {
/* FP:nonstandard_style.rs-0575 */         if let hir::TraitItemKind::Const(..) = ti.kind {
/* FP:nonstandard_style.rs-0576 */             NonUpperCaseGlobals::check_upper_case(cx, "associated constant", None, &ti.ident);
/* FP:nonstandard_style.rs-0577 */         }
/* FP:nonstandard_style.rs-0578 */     }
/* FP:nonstandard_style.rs-0579 */ 
/* FP:nonstandard_style.rs-0580 */     fn check_impl_item(&mut self, cx: &LateContext<'_>, ii: &hir::ImplItem<'_>) {
/* FP:nonstandard_style.rs-0581 */         if let hir::ImplItemKind::Const(..) = ii.kind
/* FP:nonstandard_style.rs-0582 */             && let hir::ImplItemImplKind::Inherent { .. } = ii.impl_kind
/* FP:nonstandard_style.rs-0583 */         {
/* FP:nonstandard_style.rs-0584 */             NonUpperCaseGlobals::check_upper_case(cx, "associated constant", None, &ii.ident);
/* FP:nonstandard_style.rs-0585 */         }
/* FP:nonstandard_style.rs-0586 */     }
/* FP:nonstandard_style.rs-0587 */ 
/* FP:nonstandard_style.rs-0588 */     fn check_pat(&mut self, cx: &LateContext<'_>, p: &hir::Pat<'_>) {
/* FP:nonstandard_style.rs-0589 */         // Lint for constants that look like binding identifiers (#7526)
/* FP:nonstandard_style.rs-0590 */         if let PatKind::Expr(hir::PatExpr {
/* FP:nonstandard_style.rs-0591 */             kind: PatExprKind::Path(hir::QPath::Resolved(None, path)),
/* FP:nonstandard_style.rs-0592 */             ..
/* FP:nonstandard_style.rs-0593 */         }) = p.kind
/* FP:nonstandard_style.rs-0594 */         {
/* FP:nonstandard_style.rs-0595 */             if let Res::Def(DefKind::Const, _) = path.res
/* FP:nonstandard_style.rs-0596 */                 && let [segment] = path.segments
/* FP:nonstandard_style.rs-0597 */             {
/* FP:nonstandard_style.rs-0598 */                 NonUpperCaseGlobals::check_upper_case(
/* FP:nonstandard_style.rs-0599 */                     cx,
/* FP:nonstandard_style.rs-0600 */                     "constant in pattern",
/* FP:nonstandard_style.rs-0601 */                     None,
/* FP:nonstandard_style.rs-0602 */                     &segment.ident,
/* FP:nonstandard_style.rs-0603 */                 );
/* FP:nonstandard_style.rs-0604 */             }
/* FP:nonstandard_style.rs-0605 */         }
/* FP:nonstandard_style.rs-0606 */     }
/* FP:nonstandard_style.rs-0607 */ 
/* FP:nonstandard_style.rs-0608 */     fn check_generic_param(&mut self, cx: &LateContext<'_>, param: &hir::GenericParam<'_>) {
/* FP:nonstandard_style.rs-0609 */         if let GenericParamKind::Const { .. } = param.kind {
/* FP:nonstandard_style.rs-0610 */             NonUpperCaseGlobals::check_upper_case(
/* FP:nonstandard_style.rs-0611 */                 cx,
/* FP:nonstandard_style.rs-0612 */                 "const parameter",
/* FP:nonstandard_style.rs-0613 */                 Some(param.def_id),
/* FP:nonstandard_style.rs-0614 */                 &param.name.ident(),
/* FP:nonstandard_style.rs-0615 */             );
/* FP:nonstandard_style.rs-0616 */         }
/* FP:nonstandard_style.rs-0617 */     }
/* FP:nonstandard_style.rs-0618 */ }
/* FP:nonstandard_style.rs-0619 */ 
/* FP:nonstandard_style.rs-0620 */ #[cfg(test)]