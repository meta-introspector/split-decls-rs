// Generated macro for impl_206 (impl)
macro_rules! Depcrate_dictionaryimpl_206 {
() => {
// Module: crate::dictionary
// Provides: {"impl_206"}
// Dependencies: {}
impl < KeyType : fmt :: Debug + Message , ObjectType : fmt :: Debug + Message > fmt :: Debug for NSMutableDictionary < KeyType , ObjectType > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
