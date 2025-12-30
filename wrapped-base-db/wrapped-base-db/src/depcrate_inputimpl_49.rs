// Generated macro for impl_49 (impl)
macro_rules! Depcrate_inputimpl_49 {
() => {
// Module: crate::input
// Provides: {"impl_49"}
// Dependencies: {}
impl From < & str > for LangCrateOrigin { fn from (s : & str) -> Self { match s { "alloc" => LangCrateOrigin :: Alloc , "core" => LangCrateOrigin :: Core , "proc-macro" | "proc_macro" => LangCrateOrigin :: ProcMacro , "std" => LangCrateOrigin :: Std , "test" => LangCrateOrigin :: Test , _ => LangCrateOrigin :: Other , } } }
};
}
