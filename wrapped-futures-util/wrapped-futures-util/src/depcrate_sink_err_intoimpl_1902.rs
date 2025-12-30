// Generated macro for impl_1902 (impl)
macro_rules! Depcrate_sink_err_intoimpl_1902 {
() => {
// Module: crate::sink::err_into
// Provides: {"impl_1902"}
// Dependencies: {}
impl < S , Item , E > Stream for SinkErrInto < S , Item , E > where S : Sink < Item > + Stream , S :: Error : Into < E > , { type Item = S :: Item ; delegate_stream ! (sink) ; }
};
}
