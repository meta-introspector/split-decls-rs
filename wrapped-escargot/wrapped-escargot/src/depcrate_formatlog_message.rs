// Generated macro for log_message (function)
macro_rules! Depcrate_formatlog_message {
() => {
// Module: crate::format
// Provides: {"log_message"}
// Dependencies: {}
# [cfg (feature = "print")] # [allow (clippy :: print_stderr)] pub (crate) fn log_message (msg : & Message < '_ >) { match msg { Message :: BuildFinished (ref finished) => { eprintln ! ("Build Finished: {:?}" , finished . success) ; } Message :: CompilerArtifact (ref art) => { eprintln ! ("Building {:#?}" , art . package_id ,) ; } Message :: CompilerMessage (ref comp) => { let content = comp . message . rendered . as_ref () . map (| s | s . as_ref ()) . unwrap_or_else (| | comp . message . message . as_ref ()) ; match comp . message . level { diagnostic :: DiagnosticLevel :: Ice => eprintln ! ("{content}") , diagnostic :: DiagnosticLevel :: Error => eprintln ! ("{content}") , diagnostic :: DiagnosticLevel :: Warning => eprintln ! ("{content}") , diagnostic :: DiagnosticLevel :: Note => eprintln ! ("{content}") , diagnostic :: DiagnosticLevel :: Help => eprintln ! ("{content}") , # [cfg (not (feature = "strict_unstable"))] _ => eprintln ! ("Unknown message: {:#?}" , msg) , } } Message :: BuildScriptExecuted (ref script) => { eprintln ! ("Ran script from {:#?}" , script . package_id) ; } # [cfg (not (feature = "strict_unstable"))] _ => { eprintln ! ("Unknown message: {:#?}" , msg) ; } } }
};
}
