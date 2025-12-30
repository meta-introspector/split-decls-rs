// Generated macro for impl_224 (impl)
macro_rules! Depcrate_configimpl_224 {
() => {
// Module: crate::config
// Provides: {"impl_224"}
// Dependencies: {}
impl BackendConfig { # [doc = " Parse the configuration passed in using `-Cllvm-args`."] pub fn from_opts (opts : & [String]) -> Result < Self , String > { let mut config = BackendConfig { jit_mode : false , jit_args : match std :: env :: var ("CG_CLIF_JIT_ARGS") { Ok (args) => args . split (' ') . map (| arg | arg . to_string ()) . collect () , Err (std :: env :: VarError :: NotPresent) => vec ! [] , Err (std :: env :: VarError :: NotUnicode (s)) => { panic ! ("CG_CLIF_JIT_ARGS not unicode: {:?}" , s) ; } } , } ; for opt in opts { if opt . starts_with ("-import-instr-limit") { continue ; } match & * * opt { "jit-mode" => config . jit_mode = true , _ => return Err (format ! ("Unknown option `{}`" , opt)) , } } Ok (config) } }
};
}
