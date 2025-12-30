// Generated macro for DADiskIsClaimed (function)
macro_rules! Depcrate_generatedDADiskIsClaimed {
() => {
// Module: crate::generated
// Provides: {"DADiskIsClaimed"}
// Dependencies: {}
# [cfg (feature = "DADisk")] # [deprecated = "renamed to `DADisk::is_claimed`"] # [inline] pub unsafe extern "C-unwind" fn DADiskIsClaimed (disk : & DADisk) -> bool { extern "C-unwind" { fn DADiskIsClaimed (disk : & DADisk) -> Boolean ; } let ret = unsafe { DADiskIsClaimed (disk) } ; ret != 0 }
};
}
