macro_rules! deps {
    () => {
        TargetKind!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl From < & str > for TargetKind { fn from (value : & str) -> Self { match value { "example" => TargetKind :: Example , "test" => TargetKind :: Test , "bench" => TargetKind :: Bench , "custom-build" => TargetKind :: CustomBuild , "bin" => TargetKind :: Bin , "lib" => TargetKind :: Lib , "rlib" => TargetKind :: RLib , "dylib" => TargetKind :: DyLib , "cdylib" => TargetKind :: CDyLib , "staticlib" => TargetKind :: StaticLib , "proc-macro" => TargetKind :: ProcMacro , x => TargetKind :: Unknown (x . to_string ()) , } } }
    };
}

impl_67!()