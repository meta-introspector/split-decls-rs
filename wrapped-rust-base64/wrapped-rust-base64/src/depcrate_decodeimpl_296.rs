// Generated macro for impl_296 (impl)
macro_rules! Depcrate_decodeimpl_296 {
() => {
// Module: crate::decode
// Provides: {"impl_296"}
// Dependencies: {}
# [cfg (any (feature = "std" , test))] impl error :: Error for DecodeSliceError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self { DecodeSliceError :: DecodeError (e) => Some (e) , DecodeSliceError :: OutputSliceTooSmall => None , } } }
};
}
