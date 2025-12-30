// Generated macro for impl_297 (impl)
macro_rules! Depcrateimpl_297 {
() => {
// Module: crate
// Provides: {"impl_297"}
// Dependencies: {}
impl < E : Display > Display for AllocOrInitError < E > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { AllocOrInitError :: Alloc (err) => err . fmt (f) , AllocOrInitError :: Init (err) => write ! (f , "initialization failed: {}" , err) , } } }
};
}
