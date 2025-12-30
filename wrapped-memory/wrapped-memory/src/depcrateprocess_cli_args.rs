// Generated macro for process_cli_args (function)
macro_rules! Depcrateprocess_cli_args {
() => {
// Module: crate
// Provides: {"process_cli_args"}
// Dependencies: {}
fn process_cli_args () -> ProcessedArgs { let processed = ProcessedArgs :: parse () ; if let Some (ref os) = processed . os { if ! os . chars () . all (| c | c . is_alphanumeric () || c == '_' || c == '-') { panic ! ("The OS had an unexpected character") ; } } for example in & processed . examples { if ! example . chars () . all (| c | c . is_alphanumeric () || c == '_' || c == '/') { panic ! ("An example had an unexpected character \"{example:?}\"") ; } } processed }
};
}
