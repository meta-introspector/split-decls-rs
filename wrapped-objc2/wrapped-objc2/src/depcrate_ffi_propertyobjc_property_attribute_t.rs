// Generated macro for objc_property_attribute_t (struct)
macro_rules! Depcrate_ffi_propertyobjc_property_attribute_t {
() => {
// Module: crate::ffi::property
// Provides: {"objc_property_attribute_t"}
// Dependencies: {}
# [doc = " Describes an Objective-C property attribute."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct objc_property_attribute_t { # [doc = " The name of the attribute."] pub name : * const c_char , # [doc = " The value of the attribute"] # [doc = ""] # [doc = " Usually NULL."] pub value : * const c_char , }
};
}
