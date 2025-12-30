// Generated macro for Handle (type)
macro_rules! Depcrate_baseHandle {
() => {
// Module: crate::base
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " Object Handles"] # [doc = ""] # [doc = " Handles represent access to an opaque object. Handles are untyped by default, but get a"] # [doc = " meaning when you combine them with an interface. Internally, they are simple void pointers. It"] # [doc = " is the UEFI driver model that applies meaning to them."] pub type Handle = * mut core :: ffi :: c_void ;
};
}
