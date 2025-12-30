// Generated macro for impl_449 (impl)
macro_rules! Depcrate_rustc_entryimpl_449 {
() => {
// Module: crate::rustc_entry
// Provides: {"impl_449"}
// Dependencies: {}
impl < K : Debug , V : Debug , A : Allocator > Debug for RustcOccupiedEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
};
}
