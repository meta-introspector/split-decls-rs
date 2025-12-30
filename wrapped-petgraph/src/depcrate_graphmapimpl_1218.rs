// Generated macro for impl_1218 (impl)
macro_rules! Depcrate_graphmapimpl_1218 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1218"}
// Dependencies: {}
impl < 'b , T > PartialEq for Ptr < 'b , T > { # [doc = " Ptr compares by pointer equality, i.e if they point to the same value"] fn eq (& self , other : & Ptr < 'b , T >) -> bool { ptr_eq (self . 0 , other . 0) } }
};
}
