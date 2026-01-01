/* FP:mod.rs-0001 */ #[cfg(test)]
/* FP:mod.rs-0003 */ 
/* FP:mod.rs-0005 */ use std::borrow::Cow;
/* FP:mod.rs-0006 */ 
/* FP:mod.rs-0007 */ use rustc_ast as ast;
/* FP:mod.rs-0008 */ use crate::rustc_complete::token::{Token, TokenKind};
/* FP:mod.rs-0009 */ use crate::rustc_complete::tokenstream::{TokenStream, TokenTree};
/* FP:mod.rs-0010 */ pub use state::{
/* FP:mod.rs-0011 */     AnnNode, Comments, PpAnn, PrintState, State, print_crate, print_crate_as_interface,
/* FP:mod.rs-0012 */ };
/* FP:mod.rs-0013 */ 
/* FP:mod.rs-0014 */ /// Print the token kind precisely, without converting `$crate` into its respective crate name.
/* FP:mod.rs-0015 */ pub fn token_kind_to_string(tok: &TokenKind) -> Cow<'static, str> {
/* FP:mod.rs-0016 */     State::new().token_kind_to_string(tok)
/* FP:mod.rs-0017 */ }
/* FP:mod.rs-0018 */ 
/* FP:mod.rs-0019 */ /// Print the token precisely, without converting `$crate` into its respective crate name.
/* FP:mod.rs-0020 */ pub fn token_to_string(token: &Token) -> Cow<'static, str> {
/* FP:mod.rs-0021 */     State::new().token_to_string(token)
/* FP:mod.rs-0022 */ }
/* FP:mod.rs-0023 */ 
/* FP:mod.rs-0024 */ pub fn ty_to_string(ty: &ast::Ty) -> String {
/* FP:mod.rs-0025 */     State::new().ty_to_string(ty)
/* FP:mod.rs-0026 */ }
/* FP:mod.rs-0027 */ 
/* FP:mod.rs-0028 */ pub fn bounds_to_string(bounds: &[ast::GenericBound]) -> String {
/* FP:mod.rs-0029 */     State::new().bounds_to_string(bounds)
/* FP:mod.rs-0030 */ }
/* FP:mod.rs-0031 */ 
/* FP:mod.rs-0032 */ pub fn where_bound_predicate_to_string(where_bound_predicate: &ast::WhereBoundPredicate) -> String {
/* FP:mod.rs-0033 */     State::new().where_bound_predicate_to_string(where_bound_predicate)
/* FP:mod.rs-0034 */ }
/* FP:mod.rs-0035 */ 
/* FP:mod.rs-0036 */ pub fn pat_to_string(pat: &ast::Pat) -> String {
/* FP:mod.rs-0037 */     State::new().pat_to_string(pat)
/* FP:mod.rs-0038 */ }
/* FP:mod.rs-0039 */ 
/* FP:mod.rs-0040 */ pub fn expr_to_string(e: &ast::Expr) -> String {
/* FP:mod.rs-0041 */     State::new().expr_to_string(e)
/* FP:mod.rs-0042 */ }
/* FP:mod.rs-0043 */ 
/* FP:mod.rs-0044 */ pub fn tt_to_string(tt: &TokenTree) -> String {
/* FP:mod.rs-0045 */     State::new().tt_to_string(tt)
/* FP:mod.rs-0046 */ }
/* FP:mod.rs-0047 */ 
/* FP:mod.rs-0048 */ pub fn tts_to_string(tokens: &TokenStream) -> String {
/* FP:mod.rs-0049 */     State::new().tts_to_string(tokens)
/* FP:mod.rs-0050 */ }
/* FP:mod.rs-0051 */ 
/* FP:mod.rs-0052 */ pub fn item_to_string(i: &ast::Item) -> String {
/* FP:mod.rs-0053 */     State::new().item_to_string(i)
/* FP:mod.rs-0054 */ }
/* FP:mod.rs-0055 */ 
/* FP:mod.rs-0056 */ pub fn assoc_item_to_string(i: &ast::AssocItem) -> String {
/* FP:mod.rs-0057 */     State::new().assoc_item_to_string(i)
/* FP:mod.rs-0058 */ }
/* FP:mod.rs-0059 */ 
/* FP:mod.rs-0060 */ pub fn foreign_item_to_string(i: &ast::ForeignItem) -> String {
/* FP:mod.rs-0061 */     State::new().foreign_item_to_string(i)
/* FP:mod.rs-0062 */ }
/* FP:mod.rs-0063 */ 
/* FP:mod.rs-0064 */ pub fn stmt_to_string(s: &ast::Stmt) -> String {
/* FP:mod.rs-0065 */     State::new().stmt_to_string(s)
/* FP:mod.rs-0066 */ }
/* FP:mod.rs-0067 */ 
/* FP:mod.rs-0068 */ pub fn path_to_string(p: &ast::Path) -> String {
/* FP:mod.rs-0069 */     State::new().path_to_string(p)
/* FP:mod.rs-0070 */ }
/* FP:mod.rs-0071 */ 
/* FP:mod.rs-0072 */ pub fn path_segment_to_string(p: &ast::PathSegment) -> String {
/* FP:mod.rs-0073 */     State::new().path_segment_to_string(p)
/* FP:mod.rs-0074 */ }
/* FP:mod.rs-0075 */ 
/* FP:mod.rs-0076 */ pub fn vis_to_string(v: &ast::Visibility) -> String {
/* FP:mod.rs-0077 */     State::new().vis_to_string(v)
/* FP:mod.rs-0078 */ }
/* FP:mod.rs-0079 */ 
/* FP:mod.rs-0080 */ pub fn meta_list_item_to_string(li: &ast::MetaItemInner) -> String {
/* FP:mod.rs-0081 */     State::new().meta_list_item_to_string(li)
/* FP:mod.rs-0082 */ }
/* FP:mod.rs-0083 */ 
/* FP:mod.rs-0084 */ pub fn attribute_to_string(attr: &ast::Attribute) -> String {
/* FP:mod.rs-0085 */     State::new().attribute_to_string(attr)
/* FP:mod.rs-0086 */ }
/* FP:mod.rs-0087 */ 
/* FP:mod.rs-0088 */ pub fn to_string(f: impl FnOnce(&mut State<'_>)) -> String {
/* FP:mod.rs-0089 */     State::to_string(f)
/* FP:mod.rs-0090 */ }
/* FP:mod.rs-0091 */ 
/* FP:mod.rs-0092 */ pub fn crate_to_string_for_macros(krate: &ast::Crate) -> String {
/* FP:mod.rs-0093 */     State::to_string(|s| {
/* FP:mod.rs-0094 */         s.print_inner_attributes(&krate.attrs);
/* FP:mod.rs-0095 */         for item in &krate.items {
/* FP:mod.rs-0096 */             s.print_item(item);
/* FP:mod.rs-0097 */         }
/* FP:mod.rs-0098 */     })
/* FP:mod.rs-0099 */ }