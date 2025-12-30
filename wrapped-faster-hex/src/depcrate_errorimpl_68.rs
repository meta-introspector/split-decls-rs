// Generated macro for impl_68 (impl)
macro_rules! Depcrate_errorimpl_68 {
() => {
// Module: crate::error
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (feature = "std")] impl :: std :: error :: Error for Error { fn description (& self) -> & str { match * self { Error :: InvalidChar => "invalid character" , Error :: InvalidLength (_) => "invalid length" , Error :: Overflow => "overflow" , } } }
};
}
