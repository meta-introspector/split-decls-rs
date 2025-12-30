// Generated macro for impl_350 (impl)
macro_rules! Depcrate_nostd_ioimpl_350 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_350"}
// Dependencies: {}
impl fmt :: Debug for Repr { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Repr :: Custom (ref c) => fmt :: Debug :: fmt (& c , fmt) , Repr :: Simple (kind) => fmt . debug_tuple ("Kind") . field (& kind) . finish () , } } }
};
}
