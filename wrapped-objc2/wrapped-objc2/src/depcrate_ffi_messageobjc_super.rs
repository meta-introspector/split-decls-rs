// Generated macro for objc_super (struct)
macro_rules! Depcrate_ffi_messageobjc_super {
() => {
// Module: crate::ffi::message
// Provides: {"objc_super"}
// Dependencies: {}
# [doc = " Specifies data used when sending messages to superclasses."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct objc_super { # [doc = " The object / instance to send a message to."] pub receiver : * mut AnyObject , # [doc = " The particular superclass of the instance to message."] # [doc = ""] # [doc = " Named `class` in older Objective-C versions."] pub super_class : * const AnyClass , }
};
}
