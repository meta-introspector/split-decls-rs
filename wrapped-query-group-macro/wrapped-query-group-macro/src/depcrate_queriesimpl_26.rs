// Generated macro for impl_26 (impl)
macro_rules! Depcrate_queriesimpl_26 {
() => {
// Module: crate::queries
// Provides: {"impl_26"}
// Dependencies: {}
impl ToTokens for SetterKind { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { match self { SetterKind :: Plain (input_setter) => input_setter . to_tokens (tokens) , SetterKind :: WithDurability (input_setter_with_durability) => { input_setter_with_durability . to_tokens (tokens) } } } }
};
}
