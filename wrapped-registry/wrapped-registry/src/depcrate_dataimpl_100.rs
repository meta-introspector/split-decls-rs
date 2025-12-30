// Generated macro for impl_100 (impl)
macro_rules! Depcrate_dataimpl_100 {
() => {
// Module: crate::data
// Provides: {"impl_100"}
// Dependencies: {}
impl Drop for Data { fn drop (& mut self) { if ! self . ptr . is_null () { unsafe { HeapFree (GetProcessHeap () , 0 , self . ptr as * mut _) ; } } } }
};
}
