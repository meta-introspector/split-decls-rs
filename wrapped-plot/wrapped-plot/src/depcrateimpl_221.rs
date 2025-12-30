// Generated macro for impl_221 (impl)
macro_rules! Depcrateimpl_221 {
() => {
// Module: crate
// Provides: {"impl_221"}
// Dependencies: {}
impl :: std :: error :: Error for VersionError { fn description (& self) -> & str { match self { VersionError :: Exec (_) => "Execution Error" , VersionError :: Error (_) => "Other Error" , VersionError :: OutputError => "Output Error" , VersionError :: ParseError (_) => "Parse Error" , } } fn cause (& self) -> Option < & dyn :: std :: error :: Error > { match self { VersionError :: Exec (err) => Some (err) , _ => None , } } }
};
}
