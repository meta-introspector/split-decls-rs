// Generated macro for sgx_image_base (module)
macro_rules! Depcrate_backtracesgx_image_base {
() => {
// Module: crate::backtrace
// Provides: {"sgx_image_base"}
// Dependencies: {}
# [cfg (all (target_env = "sgx" , target_vendor = "fortanix"))] mod sgx_image_base { # [cfg (not (feature = "std"))] pub (crate) mod imp { use core :: ffi :: c_void ; use core :: sync :: atomic :: { AtomicUsize , Ordering :: SeqCst } ; static IMAGE_BASE : AtomicUsize = AtomicUsize :: new (0) ; # [doc = " Set the image base address. This is only available for Fortanix SGX"] # [doc = " target when the `std` feature is not enabled. This can be used in the"] # [doc = " standard library to set the correct base address."] # [doc (hidden)] pub fn set_image_base (base_addr : * mut c_void) { IMAGE_BASE . store (base_addr as _ , SeqCst) ; } pub (crate) fn get_image_base () -> * mut c_void { IMAGE_BASE . load (SeqCst) as _ } } # [cfg (feature = "std")] mod imp { use core :: ffi :: c_void ; pub (crate) fn get_image_base () -> * mut c_void { std :: os :: fortanix_sgx :: mem :: image_base () as _ } } pub (crate) use imp :: get_image_base ; }
};
}
