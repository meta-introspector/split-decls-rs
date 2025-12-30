// Generated macro for impl_1987 (impl)
macro_rules! Depcrate_sink_with_flat_mapimpl_1987 {
() => {
// Module: crate::sink::with_flat_map
// Provides: {"impl_1987"}
// Dependencies: {}
impl < S , Item , U , St , F > Stream for WithFlatMap < S , Item , U , St , F > where S : Stream + Sink < Item > , F : FnMut (U) -> St , St : Stream < Item = Result < Item , S :: Error > > , { type Item = S :: Item ; delegate_stream ! (sink) ; }
};
}
