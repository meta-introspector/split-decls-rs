// Generated macro for impl_1988 (impl)
macro_rules! Depcrate_sink_with_flat_mapimpl_1988 {
() => {
// Module: crate::sink::with_flat_map
// Provides: {"impl_1988"}
// Dependencies: {}
impl < S , Item , U , St , F > FusedStream for WithFlatMap < S , Item , U , St , F > where S : FusedStream + Sink < Item > , F : FnMut (U) -> St , St : Stream < Item = Result < Item , S :: Error > > , { fn is_terminated (& self) -> bool { self . sink . is_terminated () } }
};
}
