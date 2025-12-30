// Generated macro for impl_600 (impl)
macro_rules! Depcrate_tableimpl_600 {
() => {
// Module: crate::table
// Provides: {"impl_600"}
// Dependencies: {}
impl < T > fmt :: Debug for IterHash < '_ , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
