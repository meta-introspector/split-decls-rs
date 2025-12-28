macro_rules! deps {
    () => {
        TargetKind!();
        Target!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Target { # [doc = " Return true if this target is of the given kind."] pub fn is_kind (& self , name : TargetKind) -> bool { self . kind . iter () . any (| kind | kind == & name) } methods_target_is_kind ! { is_lib => TargetKind :: Lib , is_bin => TargetKind :: Bin , is_example => TargetKind :: Example , is_test => TargetKind :: Test , is_bench => TargetKind :: Bench , is_custom_build => TargetKind :: CustomBuild , is_proc_macro => TargetKind :: ProcMacro , is_cdylib => TargetKind :: CDyLib , is_dylib => TargetKind :: DyLib , is_rlib => TargetKind :: RLib , is_staticlib => TargetKind :: StaticLib } }
    };
}

impl_30!()