// Generated macro for impl_107 (impl)
macro_rules! Depcrateimpl_107 {
() => {
// Module: crate
// Provides: {"impl_107"}
// Dependencies: {}
impl Target { # [doc = " Return true if this target is of the given kind."] pub fn is_kind (& self , name : TargetKind) -> bool { self . kind . iter () . any (| kind | kind == & name) } methods_target_is_kind ! { is_lib => TargetKind :: Lib , is_bin => TargetKind :: Bin , is_example => TargetKind :: Example , is_test => TargetKind :: Test , is_bench => TargetKind :: Bench , is_custom_build => TargetKind :: CustomBuild , is_proc_macro => TargetKind :: ProcMacro , is_cdylib => TargetKind :: CDyLib , is_dylib => TargetKind :: DyLib , is_rlib => TargetKind :: RLib , is_staticlib => TargetKind :: StaticLib } }
};
}
