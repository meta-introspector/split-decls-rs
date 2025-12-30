// Generated macro for SynInterpreter (struct)
macro_rules! Depcrate_syn_interpreterSynInterpreter {
() => {
// Module: crate::syn_interpreter
// Provides: {"SynInterpreter"}
// Dependencies: {}
# [doc = " Syn-based interpreter for executing split-decls-rs functions"] pub struct SynInterpreter { functions : HashMap < String , ItemFn > , variables : HashMap < String , SynValue > , call_stack : Vec < String > , }
};
}
