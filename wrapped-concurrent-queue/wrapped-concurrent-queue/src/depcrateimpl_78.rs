// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl < T > fmt :: Debug for ConcurrentQueue < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ConcurrentQueue") . field ("len" , & self . len ()) . field ("capacity" , & self . capacity ()) . field ("is_closed" , & self . is_closed ()) . finish () } }
};
}
