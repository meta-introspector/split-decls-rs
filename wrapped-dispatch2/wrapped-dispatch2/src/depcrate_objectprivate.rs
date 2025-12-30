// Generated macro for private (module)
macro_rules! Depcrate_objectprivate {
() => {
// Module: crate::object
// Provides: {"private"}
// Dependencies: {}
mod private { # [allow (non_camel_case_types)] # [repr (C)] # [derive (Debug)] pub struct dispatch_object_s { # [doc = " opaque value"] _inner : [u8 ; 0] , _p : crate :: OpaqueData , } # [cfg (feature = "objc2")] unsafe impl objc2 :: encode :: RefEncode for dispatch_object_s { const ENCODING_REF : objc2 :: encode :: Encoding = objc2 :: encode :: Encoding :: Object ; } }
};
}
