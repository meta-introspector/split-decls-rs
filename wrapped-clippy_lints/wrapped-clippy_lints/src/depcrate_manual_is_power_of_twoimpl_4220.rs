// Generated macro for impl_4220 (impl)
macro_rules! Depcrate_manual_is_power_of_twoimpl_4220 {
() => {
// Module: crate::manual_is_power_of_two
// Provides: {"impl_4220"}
// Dependencies: {}
impl ManualIsPowerOfTwo { pub fn new (conf : & 'static Conf) -> Self { Self { msrv : conf . msrv } } fn build_sugg (& self , cx : & LateContext < '_ > , expr : & Expr < '_ > , receiver : & Expr < '_ >) { if is_in_const_context (cx) && ! self . msrv . meets (cx , msrvs :: CONST_IS_POWER_OF_TWO) { return ; } let mut applicability = Applicability :: MachineApplicable ; let snippet = Sugg :: hir_with_applicability (cx , receiver , "_" , & mut applicability) ; span_lint_and_sugg (cx , MANUAL_IS_POWER_OF_TWO , expr . span , "manually reimplementing `is_power_of_two`" , "consider using `.is_power_of_two()`" , format ! ("{}.is_power_of_two()" , snippet . maybe_paren ()) , applicability ,) ; } }
};
}
