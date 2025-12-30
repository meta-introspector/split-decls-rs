// Generated macro for impl_230 (impl)
macro_rules! Depcrate_responseimpl_230 {
() => {
// Module: crate::response
// Provides: {"impl_230"}
// Dependencies: {}
impl < M , O > PartialEq for DataPayloadOr < M , O > where M : DynamicDataMarker , for < 'a > < M :: DataStruct as Yokeable < 'a > > :: Output : PartialEq , O : Eq , { fn eq (& self , other : & Self) -> bool { match (self . get () , other . get ()) { (Ok (x) , Ok (y)) => x == y , (Err (x) , Err (y)) => x == y , _ => false , } } }
};
}
