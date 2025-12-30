// Generated macro for impl_471 (impl)
macro_rules! Depcrate_pathsimpl_471 {
() => {
// Module: crate::paths
// Provides: {"impl_471"}
// Dependencies: {}
impl PathNS { fn matches (self , ns : Option < Namespace >) -> bool { let required = match self { PathNS :: Type => TypeNS , PathNS :: Value => ValueNS , PathNS :: Macro => MacroNS , PathNS :: Arbitrary => return true , } ; ns == Some (required) } }
};
}
