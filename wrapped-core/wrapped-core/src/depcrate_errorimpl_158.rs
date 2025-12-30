// Generated macro for impl_158 (impl)
macro_rules! Depcrate_errorimpl_158 {
() => {
// Module: crate::error
// Provides: {"impl_158"}
// Dependencies: {}
impl DataErrorKind { # [doc = " Converts this DataErrorKind into a DataError."] # [doc = ""] # [doc = " If possible, you should attach context using a `with_` function."] # [inline] pub const fn into_error (self) -> DataError { DataError { kind : self , marker : None , str_context : None , silent : false , } } # [doc = " Creates a DataError with a data marker context."] # [inline] pub const fn with_marker (self , marker : DataMarkerInfo) -> DataError { self . into_error () . with_marker (marker) } # [doc = " Creates a DataError with a string context."] # [inline] pub const fn with_str_context (self , context : & 'static str) -> DataError { self . into_error () . with_str_context (context) } # [doc = " Creates a DataError with a type name context."] # [inline] pub fn with_type_context < T > (self) -> DataError { self . into_error () . with_type_context :: < T > () } # [doc = " Creates a DataError with a request context."] # [inline] pub fn with_req (self , marker : DataMarkerInfo , req : DataRequest) -> DataError { self . into_error () . with_req (marker , req) } }
};
}
