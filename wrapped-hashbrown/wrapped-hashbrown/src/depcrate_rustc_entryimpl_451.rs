// Generated macro for impl_451 (impl)
macro_rules! Depcrate_rustc_entryimpl_451 {
() => {
// Module: crate::rustc_entry
// Provides: {"impl_451"}
// Dependencies: {}
impl < K : Debug , V , A : Allocator > Debug for RustcVacantEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
};
}
