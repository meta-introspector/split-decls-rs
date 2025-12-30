// Generated macro for impl_477 (impl)
macro_rules! Depcrate_back_writeimpl_477 {
() => {
// Module: crate::back::write
// Provides: {"impl_477"}
// Dependencies: {}
impl SharedEmitter { fn new () -> (SharedEmitter , SharedEmitterMain) { let (sender , receiver) = channel () ; (SharedEmitter { sender } , SharedEmitterMain { receiver }) } pub fn inline_asm_error (& self , span : SpanData , msg : String , level : Level , source : Option < (String , Vec < InnerSpan >) > ,) { drop (self . sender . send (SharedEmitterMessage :: InlineAsmError (span , msg , level , source))) ; } fn fatal (& self , msg : & str) { drop (self . sender . send (SharedEmitterMessage :: Fatal (msg . to_string ()))) ; } }
};
}
