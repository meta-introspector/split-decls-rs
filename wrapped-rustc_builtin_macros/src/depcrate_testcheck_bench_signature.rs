// Generated macro for check_bench_signature (function)
macro_rules! Depcrate_testcheck_bench_signature {
() => {
// Module: crate::test
// Provides: {"check_bench_signature"}
// Dependencies: {}
fn check_bench_signature (cx : & ExtCtxt < '_ > , i : & ast :: Item , f : & ast :: Fn ,) -> Result < () , ErrorGuaranteed > { if f . sig . decl . inputs . len () != 1 { return Err (cx . dcx () . emit_err (errors :: BenchSig { span : i . span })) ; } Ok (()) }
};
}
