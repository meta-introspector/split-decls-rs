// Generated macro for objc_method_description (struct)
macro_rules! Depcrate_ffi_methodobjc_method_description {
() => {
// Module: crate::ffi::method
// Provides: {"objc_method_description"}
// Dependencies: {}
# [doc = " Describes an Objective-C method."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct objc_method_description { # [doc = " The name of the method."] pub name : Option < Sel > , # [doc = " The types of the method arguments."] pub types : * const c_char , }
};
}
