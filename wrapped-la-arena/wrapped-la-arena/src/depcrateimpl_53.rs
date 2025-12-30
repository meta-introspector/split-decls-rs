// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for Arena < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Arena") . field ("len" , & self . len ()) . field ("data" , & self . data) . finish () } }
};
}
