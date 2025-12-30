// Generated macro for impl_1900 (impl)
macro_rules! Depcrate_sink_err_intoimpl_1900 {
() => {
// Module: crate::sink::err_into
// Provides: {"impl_1900"}
// Dependencies: {}
impl < Si , E , Item > SinkErrInto < Si , Item , E > where Si : Sink < Item > , Si :: Error : Into < E > , { pub (super) fn new (sink : Si) -> Self { Self { sink : SinkExt :: sink_map_err (sink , Into :: into) } } delegate_access_inner ! (sink , Si , (.)) ; }
};
}
