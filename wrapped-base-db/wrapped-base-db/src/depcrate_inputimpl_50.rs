// Generated macro for impl_50 (impl)
macro_rules! Depcrate_inputimpl_50 {
() => {
// Module: crate::input
// Provides: {"impl_50"}
// Dependencies: {}
impl fmt :: Display for LangCrateOrigin { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let text = match self { LangCrateOrigin :: Alloc => "alloc" , LangCrateOrigin :: Core => "core" , LangCrateOrigin :: ProcMacro => "proc_macro" , LangCrateOrigin :: Std => "std" , LangCrateOrigin :: Test => "test" , LangCrateOrigin :: Other => "other" , } ; f . write_str (text) } }
};
}
