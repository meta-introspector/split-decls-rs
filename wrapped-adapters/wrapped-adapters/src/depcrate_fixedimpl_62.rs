// Generated macro for impl_62 (impl)
macro_rules! Depcrate_fixedimpl_62 {
() => {
// Module: crate::fixed
// Provides: {"impl_62"}
// Dependencies: {}
impl < M : DataMarker > FixedProvider < M > { # [doc = " Creates a `FixedProvider` with an owned (allocated) payload of the given data."] pub fn from_owned (data : M :: DataStruct) -> Self { Self :: from_payload (DataPayload :: from_owned (data)) } # [doc = " Creates a `FixedProvider` with a statically borrowed payload of the given data."] pub fn from_static (data : & 'static M :: DataStruct) -> Self { FixedProvider { data : DataPayload :: from_static_ref (data) , } } # [doc = " Creates a `FixedProvider` from an existing [`DataPayload`]."] pub fn from_payload (data : DataPayload < M >) -> Self { FixedProvider { data } } # [doc = " Creates a `FixedProvider` with the default (allocated) version of the data struct."] pub fn new_default () -> Self where M :: DataStruct : Default , { Self :: from_owned (M :: DataStruct :: default ()) } }
};
}
