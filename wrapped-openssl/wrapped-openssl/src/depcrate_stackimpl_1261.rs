// Generated macro for impl_1261 (impl)
macro_rules! Depcrate_stackimpl_1261 {
() => {
// Module: crate::stack
// Provides: {"impl_1261"}
// Dependencies: {}
impl < T > fmt :: Debug for Stack < T > where T : Stackable , T :: Ref : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_list () . entries (self) . finish () } }
};
}
