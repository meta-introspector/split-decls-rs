// Generated macro for impl_1536 (impl)
macro_rules! Depcrate_registryimpl_1536 {
() => {
// Module: crate::registry
// Provides: {"impl_1536"}
// Dependencies: {}
impl Display for MetaTypeName < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { MetaTypeName :: Named (name) => write ! (f , "{}" , name) , MetaTypeName :: NonNull (name) => write ! (f , "{}!" , name) , MetaTypeName :: List (name) => write ! (f , "[{}]" , name) , } } }
};
}
