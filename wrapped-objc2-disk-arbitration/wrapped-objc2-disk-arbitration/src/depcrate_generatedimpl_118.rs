// Generated macro for impl_118 (impl)
macro_rules! Depcrate_generatedimpl_118 {
() => {
// Module: crate::generated
// Provides: {"impl_118"}
// Dependencies: {}
# [cfg (feature = "DADisk")] impl DADisk { # [doc = " Renames the volume at the specified disk object."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `options`: The rename options."] # [doc = ""] # [doc = " Parameter `callback`: The callback function to call once the rename completes."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter to pass to the callback function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `callback` must be implemented correctly."] # [doc = " - `context` must be a valid pointer or null."] # [doc (alias = "DADiskRename")] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] # [inline] pub unsafe fn rename (& self , name : & CFString , options : DADiskRenameOptions , callback : DADiskRenameCallback , context : * mut c_void ,) { extern "C-unwind" { fn DADiskRename (disk : & DADisk , name : & CFString , options : DADiskRenameOptions , callback : DADiskRenameCallback , context : * mut c_void ,) ; } unsafe { DADiskRename (self , name , options , callback , context) } } }
};
}
