// Generated macro for impl_109 (impl)
macro_rules! Depcrateimpl_109 {
() => {
// Module: crate
// Provides: {"impl_109"}
// Dependencies: {}
impl From < & str > for TargetKind { fn from (value : & str) -> Self { match value { "example" => TargetKind :: Example , "test" => TargetKind :: Test , "bench" => TargetKind :: Bench , "custom-build" => TargetKind :: CustomBuild , "bin" => TargetKind :: Bin , "lib" => TargetKind :: Lib , "rlib" => TargetKind :: RLib , "dylib" => TargetKind :: DyLib , "cdylib" => TargetKind :: CDyLib , "staticlib" => TargetKind :: StaticLib , "proc-macro" => TargetKind :: ProcMacro , x => TargetKind :: Unknown (x . to_string ()) , } } }
};
}
