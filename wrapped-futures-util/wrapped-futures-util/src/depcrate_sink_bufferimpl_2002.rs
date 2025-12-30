// Generated macro for impl_2002 (impl)
macro_rules! Depcrate_sink_bufferimpl_2002 {
() => {
// Module: crate::sink::buffer
// Provides: {"impl_2002"}
// Dependencies: {}
impl < S , Item > FusedStream for Buffer < S , Item > where S : Sink < Item > + FusedStream , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
};
}
