// Generated macro for impl_492 (impl)
macro_rules! Depcrate_common_originimpl_492 {
() => {
// Module: crate::common::origin
// Provides: {"impl_492"}
// Dependencies: {}
impl fmt :: Display for Origin { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { OriginOrNull :: Origin (ref scheme , ref auth) => write ! (f , "{}://{}" , scheme , auth) , OriginOrNull :: Null => f . write_str ("null") , } } }
};
}
