// Generated macro for impl_132 (impl)
macro_rules! Depcrate_generatedimpl_132 {
() => {
// Module: crate::generated
// Provides: {"impl_132"}
// Dependencies: {}
# [cfg (feature = "DADisk")] impl DADisk { # [doc = " Obtains the options for the specified disk."] # [doc = ""] # [doc = " Parameter `disk`: The disk object for which to obtain the options."] # [doc = ""] # [doc = " Returns: The options."] # [doc (alias = "DADiskGetOptions")] # [cfg (feature = "DADisk")] # [inline] pub unsafe fn options (& self) -> DADiskOptions { extern "C-unwind" { fn DADiskGetOptions (disk : & DADisk) -> DADiskOptions ; } unsafe { DADiskGetOptions (self) } } # [doc = " Sets the options for the specified disk."] # [doc = ""] # [doc = " Parameter `disk`: The disk object for which to set the options."] # [doc = ""] # [doc = " Parameter `options`: The options to set or clear."] # [doc = ""] # [doc = " Parameter `value`: Pass TRUE to set options; otherwise pass FALSE to clear options."] # [doc = ""] # [doc = " Returns: A result code."] # [doc (alias = "DADiskSetOptions")] # [cfg (all (feature = "DADisk" , feature = "DADissenter" , feature = "libc"))] # [inline] pub unsafe fn set_options (& self , options : DADiskOptions , value : bool) -> DAReturn { extern "C-unwind" { fn DADiskSetOptions (disk : & DADisk , options : DADiskOptions , value : Boolean) -> DAReturn ; } unsafe { DADiskSetOptions (self , options , value as _) } } }
};
}
