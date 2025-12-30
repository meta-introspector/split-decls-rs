// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl std :: error :: Error for Error { fn description (& self) -> & str { match self . kind { ErrorKind :: Regex (_) => "regex error" , } } }
};
}
