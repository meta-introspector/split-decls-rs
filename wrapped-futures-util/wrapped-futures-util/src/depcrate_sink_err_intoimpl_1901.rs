// Generated macro for impl_1901 (impl)
macro_rules! Depcrate_sink_err_intoimpl_1901 {
() => {
// Module: crate::sink::err_into
// Provides: {"impl_1901"}
// Dependencies: {}
impl < Si , Item , E > Sink < Item > for SinkErrInto < Si , Item , E > where Si : Sink < Item > , Si :: Error : Into < E > , { type Error = E ; delegate_sink ! (sink , Item) ; }
};
}
