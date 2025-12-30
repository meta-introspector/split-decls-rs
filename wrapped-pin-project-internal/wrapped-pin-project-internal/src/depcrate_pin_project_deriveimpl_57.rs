// Generated macro for impl_57 (impl)
macro_rules! Depcrate_pin_project_deriveimpl_57 {
() => {
// Module: crate::pin_project::derive
// Provides: {"impl_57"}
// Dependencies: {}
impl GenerateTokens { fn extend (& mut self , expose : bool , tokens : TokenStream) { if expose { self . exposed . extend (tokens) ; } else { self . scoped . extend (tokens) ; } } fn into_tokens (self , cx : & Context < '_ >) -> TokenStream { let mut tokens = self . exposed ; let scoped = self . scoped ; let unpin_impl = make_unpin_impl (cx) ; let drop_impl = make_drop_impl (cx) ; let allowed_lints = global_allowed_lints () ; tokens . extend (quote ! { # [allow (unused_qualifications , # allowed_lints clippy :: elidable_lifetime_names , clippy :: missing_const_for_fn , clippy :: needless_lifetimes , clippy :: semicolon_if_nothing_returned , clippy :: use_self , clippy :: used_underscore_binding)] const _ : () = { # [allow (unused_extern_crates)] extern crate pin_project as _pin_project ; # scoped # unpin_impl # drop_impl } ; }) ; tokens } }
};
}
