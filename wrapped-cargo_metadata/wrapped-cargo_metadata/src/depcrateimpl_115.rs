// Generated macro for impl_115 (impl)
macro_rules! Depcrateimpl_115 {
() => {
// Module: crate
// Provides: {"impl_115"}
// Dependencies: {}
impl fmt :: Display for CrateType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Bin => "bin" . fmt (f) , Self :: CDyLib => "cdylib" . fmt (f) , Self :: DyLib => "dylib" . fmt (f) , Self :: Lib => "lib" . fmt (f) , Self :: ProcMacro => "proc-macro" . fmt (f) , Self :: RLib => "rlib" . fmt (f) , Self :: StaticLib => "staticlib" . fmt (f) , Self :: Unknown (x) => x . fmt (f) , } } }
};
}
