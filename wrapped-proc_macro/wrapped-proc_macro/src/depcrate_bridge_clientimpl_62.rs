// Generated macro for impl_62 (impl)
macro_rules! Depcrate_bridge_clientimpl_62 {
() => {
// Module: crate::bridge::client
// Provides: {"impl_62"}
// Dependencies: {}
impl ProcMacro { pub fn name (& self) -> & 'static str { match self { ProcMacro :: CustomDerive { trait_name , .. } => trait_name , ProcMacro :: Attr { name , .. } => name , ProcMacro :: Bang { name , .. } => name , } } pub const fn custom_derive (trait_name : & 'static str , attributes : & 'static [& 'static str] , expand : impl Fn (crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { ProcMacro :: CustomDerive { trait_name , attributes , client : Client :: expand1 (expand) } } pub const fn attr (name : & 'static str , expand : impl Fn (crate :: TokenStream , crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { ProcMacro :: Attr { name , client : Client :: expand2 (expand) } } pub const fn bang (name : & 'static str , expand : impl Fn (crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { ProcMacro :: Bang { name , client : Client :: expand1 (expand) } } }
};
}
