// Generated macro for impl_32 (impl)
macro_rules! Depcrate_specimpl_32 {
() => {
// Module: crate::spec
// Provides: {"impl_32"}
// Dependencies: {}
impl Spec { pub fn insert (& mut self , value : Node) -> bool { self . 0 . insert (value) } pub fn records (& self , path : TokenStream) -> TokenStream { let mut stream = TokenStream :: default () ; for n in & self . 0 { let name = n . name () ; let symb = n . symbol () ; stream . extend (quote ! { (# path ::# symb , # name) , }) } stream } pub fn module (& self , spec : & Ident) -> TokenStream { let mut defs = TokenStream :: default () ; for n in & self . 0 { defs . extend (n . definition ()) } quote ! { pub mod # spec { # defs } } } }
};
}
