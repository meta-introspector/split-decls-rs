// Generated macro for enable_verifier (function)
macro_rules! Depcrateenable_verifier {
() => {
// Module: crate
// Provides: {"enable_verifier"}
// Dependencies: {}
# [doc = " Determine if the Cranelift ir verifier should run."] # [doc = ""] # [doc = " Returns true when `-Zverify-llvm-ir` is passed, the `CG_CLIF_ENABLE_VERIFIER` env var is set to"] # [doc = " 1 or when cg_clif is compiled with debug assertions enabled or false otherwise."] fn enable_verifier (sess : & Session) -> bool { sess . verify_llvm_ir () || cfg ! (debug_assertions) || env :: var ("CG_CLIF_ENABLE_VERIFIER") . as_deref () == Ok ("1") }
};
}
