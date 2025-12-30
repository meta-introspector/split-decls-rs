// Generated macro for link_to (function)
macro_rules! Depcrate_parserlink_to {
() => {
// Module: crate::parser
// Provides: {"link_to"}
// Dependencies: {}
pub fn link_to (opts : BindgenAttrs) -> Result < ast :: LinkToModule , Diagnostic > { let mut program = ast :: Program :: default () ; let module = module_from_opts (& mut program , & opts) ? . ok_or_else (| | { Diagnostic :: span_error (Span :: call_site () , "`link_to!` requires a module.") }) ? ; if let ast :: ImportModule :: Named (p , s) | ast :: ImportModule :: RawNamed (p , s) = & module { if ! p . starts_with ("./") && ! p . starts_with ("../") && ! p . starts_with ('/') { return Err (Diagnostic :: span_error (* s , "`link_to!` does not support module paths." ,)) ; } } opts . enforce_used () ? ; program . linked_modules . push (module) ; Ok (ast :: LinkToModule (program)) }
};
}
