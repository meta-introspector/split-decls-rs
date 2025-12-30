// Generated macro for impl_437 (impl)
macro_rules! Depcrate_render_apply_argumentsimpl_437 {
() => {
// Module: crate::render::apply_arguments
// Provides: {"impl_437"}
// Dependencies: {}
impl ImplFutureArg for FnArg { fn impl_future_arg (& mut self , anonymous_lt : & mut usize) -> Option < Lifetime > { let lifetime_id = self . maybe_ident () . map (| id | format_ident ! ("_{}" , id)) . unwrap_or_else (| | { * anonymous_lt += 1 ; format_ident ! ("_anonymous_lt_{}" , anonymous_lt) }) ; match self . as_mut_future_impl_type () { Some (ty) => { let lifetime = update_type_with_lifetime (ty , lifetime_id) ; * ty = parse_quote ! { impl core :: future :: Future < Output = # ty > } ; self . remove_mutability () ; lifetime } None => None , } } }
};
}
