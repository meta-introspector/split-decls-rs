// Generated macro for impl_1903 (impl)
macro_rules! Depcrate_sink_err_intoimpl_1903 {
() => {
// Module: crate::sink::err_into
// Provides: {"impl_1903"}
// Dependencies: {}
impl < S , Item , E > FusedStream for SinkErrInto < S , Item , E > where S : Sink < Item > + FusedStream , S :: Error : Into < E > , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
};
}
