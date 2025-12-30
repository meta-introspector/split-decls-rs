// Generated macro for macro_1984 (macro)
macro_rules! Depcrate_sink_with_flat_mapmacro_1984 {
() => {
// Module: crate::sink::with_flat_map
// Provides: {"macro_1984"}
// Dependencies: {}
pin_project ! { # [doc = " Sink for the [`with_flat_map`](super::SinkExt::with_flat_map) method."] # [must_use = "sinks do nothing unless polled"] pub struct WithFlatMap < Si , Item , U , St , F > { # [pin] sink : Si , f : F , # [pin] stream : Option < St >, buffer : Option < Item >, _marker : PhantomData < fn (U) >, } }
};
}
