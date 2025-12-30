// Generated macro for impl_113 (impl)
macro_rules! Depcrateimpl_113 {
() => {
// Module: crate
// Provides: {"impl_113"}
// Dependencies: {}
impl From < & str > for CrateType { fn from (value : & str) -> Self { match value { "bin" => CrateType :: Bin , "lib" => CrateType :: Lib , "rlib" => CrateType :: RLib , "dylib" => CrateType :: DyLib , "cdylib" => CrateType :: CDyLib , "staticlib" => CrateType :: StaticLib , "proc-macro" => CrateType :: ProcMacro , x => CrateType :: Unknown (x . to_string ()) , } } }
};
}
