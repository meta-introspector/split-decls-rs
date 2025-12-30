// Generated macro for impl_229 (impl)
macro_rules! Depcrate_responseimpl_229 {
() => {
// Module: crate::response
// Provides: {"impl_229"}
// Dependencies: {}
impl < M > PartialEq for DataPayload < M > where M : DynamicDataMarker , for < 'a > < M :: DataStruct as Yokeable < 'a > > :: Output : PartialEq , { fn eq (& self , other : & Self) -> bool { self . get () == other . get () } }
};
}
