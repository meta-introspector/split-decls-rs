// Generated macro for impl_63 (impl)
macro_rules! Depcrate_ifacedescimpl_63 {
() => {
// Module: crate::ifacedesc
// Provides: {"impl_63"}
// Dependencies: {}
impl < A : 'static > SignalBuilder < '_ , A > { pub fn annotate < N : Into < String > , V : Into < String > > (& mut self , name : N , value : V) -> & mut Self { self . desc . annotations . insert (name , value) ; self } pub fn deprecated (& mut self) -> & mut Self { self . annotate (DEPRECATED , "true") } }
};
}
