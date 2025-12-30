// Generated macro for SharedEmitterMessage (enum)
macro_rules! Depcrate_back_writeSharedEmitterMessage {
() => {
// Module: crate::back::write
// Provides: {"SharedEmitterMessage"}
// Dependencies: {}
enum SharedEmitterMessage { Diagnostic (Diagnostic) , InlineAsmError (SpanData , String , Level , Option < (String , Vec < InnerSpan >) >) , Fatal (String) , }
};
}
