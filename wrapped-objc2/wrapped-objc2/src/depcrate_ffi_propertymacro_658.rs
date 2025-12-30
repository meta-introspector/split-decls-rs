// Generated macro for macro_658 (macro)
macro_rules! Depcrate_ffi_propertymacro_658 {
() => {
// Module: crate::ffi::property
// Provides: {"macro_658"}
// Dependencies: {}
extern_c ! { # [cfg (any (doc , not (feature = "unstable-objfw")))] # [doc = " The returned array is deallocated with [`free`][crate::ffi::free]."] pub fn property_copyAttributeList (property : * const objc_property , out_len : * mut c_uint ,) -> * mut objc_property_attribute_t ; # [cfg (any (doc , not (feature = "unstable-objfw")))] pub fn property_copyAttributeValue (property : * const objc_property , attribute_name : * const c_char ,) -> * mut c_char ; # [cfg (any (doc , not (feature = "unstable-objfw")))] pub fn property_getAttributes (property : * const objc_property) -> * const c_char ; # [cfg (any (doc , not (feature = "unstable-objfw")))] pub fn property_getName (property : * const objc_property) -> * const c_char ; }
};
}
