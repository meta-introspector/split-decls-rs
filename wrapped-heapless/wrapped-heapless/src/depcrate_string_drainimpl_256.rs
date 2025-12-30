// Generated macro for impl_256 (impl)
macro_rules! Depcrate_string_drainimpl_256 {
() => {
// Module: crate::string::drain
// Provides: {"impl_256"}
// Dependencies: {}
impl < LenT : LenType > fmt :: Debug for Drain < '_ , LenT > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . as_str ()) . finish () } }
};
}
