// Generated macro for validate_int (function)
macro_rules! Depcrate_test_traitsvalidate_int {
() => {
// Module: crate::test_traits
// Provides: {"validate_int"}
// Dependencies: {}
fn validate_int < I , Input > (actual : I , expected : I , input : Input , ctx : & CheckCtx) -> TestResult where I : Int + Hex , Input : Hex + fmt :: Debug , SpecialCase : MaybeOverride < Input > , { let (result , xfail_msg) = match SpecialCase :: check_int (input , actual , expected , ctx) { _ if ctx . gen_kind == GeneratorKind :: List => (actual == expected , None) , CheckAction :: AssertSuccess => (actual == expected , None) , CheckAction :: AssertFailure (msg) => (actual != expected , Some (msg)) , CheckAction :: Custom (res) => return res , CheckAction :: Skip => return Ok (()) , CheckAction :: AssertWithUlp (_) => panic ! ("ulp has no meaning for integer checks") , } ; let make_xfail_msg = | | match xfail_msg { Some (m) => format ! ("expected failure but test passed. Does an XFAIL need to be updated?\n\
            failed at: {m}" ,) , None => String :: new () , } ; anyhow :: ensure ! (result , "\
        \n    input:    {input:?} {ibits}\
        \n    expected: {expected:<22?} {expbits}\
        \n    actual:   {actual:<22?} {actbits}\
        \n    {msg}\
        " , actbits = actual . hex () , expbits = expected . hex () , ibits = input . hex () , msg = make_xfail_msg ()) ; Ok (()) }
};
}
