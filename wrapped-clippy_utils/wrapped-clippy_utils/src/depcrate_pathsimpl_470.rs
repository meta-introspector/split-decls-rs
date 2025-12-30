// Generated macro for impl_470 (impl)
macro_rules! Depcrate_pathsimpl_470 {
() => {
// Module: crate::paths
// Provides: {"impl_470"}
// Dependencies: {}
impl PathNS { fn matches (self , ns : Option < Namespace >) -> bool { let required = match self { PathNS :: Type => TypeNS , PathNS :: Value => ValueNS , PathNS :: Macro => MacroNS , PathNS :: Arbitrary => return true , } ; ns == Some (required) } }
};
}
