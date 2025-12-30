// Generated macro for impl_183 (impl)
macro_rules! Depcrate_errorimpl_183 {
() => {
// Module: crate::error
// Provides: {"impl_183"}
// Dependencies: {}
impl Context { # [doc = " Add a non-fatal error to the context."] pub fn error < T : Display > (& mut self , msg : T) { self . errors . push (msg . to_string ()) ; } # [doc = " Add an error to the context and produce an erroring"] # [doc = " computation that will halt the macro."] pub fn fatal < T : Display , A > (& mut self , msg : T) -> DeriveResult < A > { self . error (msg) ; Err (Fatal) } # [doc = " Consume the context and if there were any errors,"] # [doc = " emit `compile_error!(..)` such that the crate using"] # [doc = " `#[derive(Arbitrary)]` will fail to compile."] pub fn check (mut self) -> Result < () , TokenStream > { fn compile_error (msg : & str) -> TokenStream { quote ! { compile_error ! (# msg) ; } } match self . errors . len () { 0 => Ok (()) , 1 => Err (compile_error (& self . errors . pop () . unwrap ())) , n => { let mut msg = format ! ("{} errors:" , n) ; for err in self . errors { msg . push_str ("\n\t# ") ; msg . push_str (& err) ; } Err (compile_error (& msg)) } } } }
};
}
