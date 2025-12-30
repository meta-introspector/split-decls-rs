// Generated macro for impl_2721 (impl)
macro_rules! Depcrate_valueimpl_2721 {
() => {
// Module: crate::value
// Provides: {"impl_2721"}
// Dependencies: {}
# [doc = " Creation methods."] impl NSValue { # [doc = " Create a new `NSValue` containing the given type."] # [doc = ""] # [doc = " Be careful when using this since you may accidentally pass a reference"] # [doc = " when you wanted to pass a concrete type instead."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Create an `NSValue` containing an `i32`."] # [doc = ""] # [doc = " ```"] # [doc = " use objc2_foundation::NSValue;"] # [doc = ""] # [doc = " let val = NSValue::new(42i32);"] # [doc = " ```"] # [doc = ""] # [doc = " [`NSPoint`]: crate::NSPoint"] pub fn new < T : 'static + Copy + Encode > (value : T) -> Retained < Self > { let bytes : NonNull < T > = NonNull :: from (& value) ; let encoding = CString :: new (T :: ENCODING . to_string ()) . unwrap () ; unsafe { Self :: initWithBytes_objCType (Self :: alloc () , bytes . cast () , NonNull :: new (encoding . as_ptr () as * mut _) . unwrap () ,) } } }
};
}
