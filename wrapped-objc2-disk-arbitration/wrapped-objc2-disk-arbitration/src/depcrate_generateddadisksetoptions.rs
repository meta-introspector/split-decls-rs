// Generated macro for DADiskSetOptions (function)
macro_rules! Depcrate_generatedDADiskSetOptions {
() => {
// Module: crate::generated
// Provides: {"DADiskSetOptions"}
// Dependencies: {}
# [cfg (all (feature = "DADisk" , feature = "DADissenter" , feature = "libc"))] # [deprecated = "renamed to `DADisk::set_options`"] # [inline] pub unsafe extern "C-unwind" fn DADiskSetOptions (disk : & DADisk , options : DADiskOptions , value : bool ,) -> DAReturn { extern "C-unwind" { fn DADiskSetOptions (disk : & DADisk , options : DADiskOptions , value : Boolean) -> DAReturn ; } unsafe { DADiskSetOptions (disk , options , value as _) } }
};
}
