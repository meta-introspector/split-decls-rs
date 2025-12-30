// Generated macro for impl_60 (impl)
macro_rules! Depcrate_ifacedescimpl_60 {
() => {
// Module: crate::ifacedesc
// Provides: {"impl_60"}
// Dependencies: {}
impl MethodDesc { pub fn annotate < N : Into < String > , V : Into < String > > (& mut self , name : N , value : V) -> & mut Self { self . annotations . insert (name , value) ; self } pub fn deprecated (& mut self) -> & mut Self { self . annotate (DEPRECATED , "true") } }
};
}
