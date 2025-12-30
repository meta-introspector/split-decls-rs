// Generated macro for impl_60 (impl)
macro_rules! Depcrate_err_parseimpl_60 {
() => {
// Module: crate::err_parse
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > ParseResultBase < T > for ErrParse < T > { fn handle_failure (& mut self , tracker : & mut DynMTrackerTrait ! ()) { for failure in & self . failures { tracker . build_failure (rustc_ast :: token :: Token :: dummy () , failure . lo . lo () . 0 , "macro parsing failure" ,) ; } } fn is_ok (& self) -> bool { false } fn unwrap (self) -> Option < T > { None } }
};
}
