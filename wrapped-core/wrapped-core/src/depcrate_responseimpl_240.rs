// Generated macro for impl_240 (impl)
macro_rules! Depcrate_responseimpl_240 {
() => {
// Module: crate::response
// Provides: {"impl_240"}
// Dependencies: {}
impl < M > DataResponse < M > where M : DynamicDataMarker , { # [doc = " Convert between two [`DynamicDataMarker`] types that are compatible with each other"] # [doc = " with compile-time type checking."] # [doc = ""] # [doc = " This happens if they both have the same [`DynamicDataMarker::DataStruct`] type."] # [doc = ""] # [doc = " Can be used to erase the marker of a data payload in cases where multiple markers correspond"] # [doc = " to the same data struct."] # [doc = ""] # [doc = " For runtime dynamic casting, use [`DataResponse::dynamic_cast()`]."] # [inline] pub fn cast < M2 > (self) -> DataResponse < M2 > where M2 : DynamicDataMarker < DataStruct = M :: DataStruct > , { DataResponse { metadata : self . metadata , payload : self . payload . cast () , } } # [doc = " Convert a [`DataResponse`] to one of the same type with runtime type checking."] # [doc = ""] # [doc = " Primarily useful to convert from a generic to a concrete marker type."] # [doc = ""] # [doc = " If the `M2` type argument does not match the true marker type, a `DataError` is returned."] # [doc = ""] # [doc = " For compile-time static casting, use [`DataResponse::cast()`]."] # [inline] pub fn dynamic_cast < M2 > (self) -> Result < DataResponse < M2 > , DataError > where M2 : DynamicDataMarker , { Ok (DataResponse { metadata : self . metadata , payload : self . payload . dynamic_cast () ? , }) } }
};
}
