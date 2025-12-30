// Generated macro for impl_588 (impl)
macro_rules! Depcrate_tableimpl_588 {
() => {
// Module: crate::table
// Provides: {"impl_588"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for Iter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
