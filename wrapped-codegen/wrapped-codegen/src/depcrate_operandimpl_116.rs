// Generated macro for impl_116 (impl)
macro_rules! Depcrate_operandimpl_116 {
() => {
// Module: crate::operand
// Provides: {"impl_116"}
// Dependencies: {}
impl Operand { pub fn tokens (& self) -> & TokenStream { match self { Borrowed (n) | Owned (n) => n , } } pub fn ref_tokens (& self) -> TokenStream { match self { Borrowed (n) => n . clone () , Owned (n) => quote ! (&# n) , } } pub fn ref_mut_tokens (& self) -> TokenStream { match self { Borrowed (n) => n . clone () , Owned (n) => quote ! (& mut # n) , } } pub fn owned_tokens (& self) -> TokenStream { match self { Borrowed (n) => quote ! (*# n) , Owned (n) => n . clone () , } } }
};
}
