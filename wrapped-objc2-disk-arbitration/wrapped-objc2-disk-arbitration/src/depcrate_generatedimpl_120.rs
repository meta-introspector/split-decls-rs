// Generated macro for impl_120 (impl)
macro_rules! Depcrate_generatedimpl_120 {
() => {
// Module: crate::generated
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (feature = "DADisk")] impl DADisk { # [doc = " Unmounts the volume at the specified disk object."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `options`: The unmount options."] # [doc = ""] # [doc = " Parameter `callback`: The callback function to call once the unmount completes."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter to pass to the callback function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `callback` must be implemented correctly."] # [doc = " - `context` must be a valid pointer or null."] # [doc (alias = "DADiskUnmount")] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] # [inline] pub unsafe fn unmount (& self , options : DADiskUnmountOptions , callback : DADiskUnmountCallback , context : * mut c_void ,) { extern "C-unwind" { fn DADiskUnmount (disk : & DADisk , options : DADiskUnmountOptions , callback : DADiskUnmountCallback , context : * mut c_void ,) ; } unsafe { DADiskUnmount (self , options , callback , context) } } }
};
}
