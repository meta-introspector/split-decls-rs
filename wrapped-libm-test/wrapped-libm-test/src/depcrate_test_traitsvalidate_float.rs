// Generated macro for validate_float (function)
macro_rules! Depcrate_test_traitsvalidate_float {
() => {
// Module: crate::test_traits
// Provides: {"validate_float"}
// Dependencies: {}
fn validate_float < F , Input > (actual : F , expected : F , input : Input , ctx : & CheckCtx) -> TestResult where F : Float + Hex , Input : Hex + fmt :: Debug , u32 : TryFrom < F :: SignedInt , Error : fmt :: Debug > , SpecialCase : MaybeOverride < Input > , { let mut assert_failure_msg = None ; let mut inner = | | -> TestResult { let mut allowed_ulp = ctx . ulp ; match SpecialCase :: check_float (input , actual , expected , ctx) { _ if ctx . gen_kind == GeneratorKind :: List => () , CheckAction :: AssertSuccess => () , CheckAction :: AssertFailure (msg) => assert_failure_msg = Some (msg) , CheckAction :: Custom (res) => return res , CheckAction :: Skip => return Ok (()) , CheckAction :: AssertWithUlp (ulp_override) => allowed_ulp = ulp_override , } ; if actual . is_nan () && expected . is_nan () { let skip_nan_biteq = ctx . basis == CheckBasis :: Mpfr || (ctx . basis == CheckBasis :: Musl && ctx . gen_kind != GeneratorKind :: List) ; if ! skip_nan_biteq { ensure ! (actual . biteq (expected) , "mismatched NaN bitpatterns") ; } return Ok (()) ; } else if actual . is_nan () || expected . is_nan () { bail ! ("real value != NaN") } let act_sig = actual . signum () ; let exp_sig = expected . signum () ; ensure ! (act_sig == exp_sig , "mismatched signs {act_sig:?} {exp_sig:?}") ; if actual . is_infinite () ^ expected . is_infinite () { bail ! ("mismatched infinities") ; } let act_bits = actual . to_bits () . signed () ; let exp_bits = expected . to_bits () . signed () ; let ulp_diff = act_bits . checked_sub (exp_bits) . unwrap () . abs () ; let ulp_u32 = u32 :: try_from (ulp_diff) . map_err (| e | anyhow ! ("{e:?}: ulp of {ulp_diff} exceeds u32::MAX")) ? ; ensure ! (ulp_u32 <= allowed_ulp , "ulp {ulp_diff} > {allowed_ulp}" ,) ; Ok (()) } ; let mut res = inner () ; if let Some (msg) = assert_failure_msg { if res . is_ok () { let e = anyhow ! ("expected failure but test passed. Does an XFAIL need to be updated?\n\
                failed at: {msg}" ,) ; res = Err (e) } else { res = Ok (()) } } res . with_context (| | { format ! ("\
            \n    input:    {input:?}\
            \n    as hex:   {ihex}\
            \n    as bits:  {ibits}\
            \n    expected: {expected:<22?} {exphex} {expbits}\
            \n    actual:   {actual:<22?} {acthex} {actbits}\
            " , ihex = input . hexf () , ibits = input . hex () , exphex = expected . hexf () , expbits = expected . hex () , actbits = actual . hex () , acthex = actual . hexf () ,) }) }
};
}
