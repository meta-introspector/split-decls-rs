// Generated macro for impl_124 (impl)
macro_rules! Depcrate_generatedimpl_124 {
() => {
// Module: crate::generated
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "DADisk")] impl DADisk { # [doc = " Ejects the specified disk object."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `options`: The eject options."] # [doc = ""] # [doc = " Parameter `callback`: The callback function to call once the ejection completes."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter to pass to the callback function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `callback` must be implemented correctly."] # [doc = " - `context` must be a valid pointer or null."] # [doc (alias = "DADiskEject")] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] # [inline] pub unsafe fn eject (& self , options : DADiskEjectOptions , callback : DADiskEjectCallback , context : * mut c_void ,) { extern "C-unwind" { fn DADiskEject (disk : & DADisk , options : DADiskEjectOptions , callback : DADiskEjectCallback , context : * mut c_void ,) ; } unsafe { DADiskEject (self , options , callback , context) } } }
};
}
