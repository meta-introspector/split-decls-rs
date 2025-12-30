// Generated macro for return_impl_trait (macro)
macro_rules! Depcrate_macrosreturn_impl_trait {
() => {
// Module: crate::macros
// Provides: {"return_impl_trait"}
// Dependencies: {}
macro_rules ! return_impl_trait { ($ (# [$ attr : meta]) * $ vis : vis fn $ name : ident <$ param : ident > $ args : tt -> $ impl_trait : ty [$ concrete : ty] $ ($ body : tt) +) => { # [cfg (not (docsrs))] $ (# [$ attr]) * $ vis fn $ name <$ param > $ args -> $ concrete $ ($ body) + # [cfg (docsrs)] $ (# [$ attr]) * $ vis fn $ name <$ param > $ args -> $ impl_trait $ ($ body) + } ; }
};
}
