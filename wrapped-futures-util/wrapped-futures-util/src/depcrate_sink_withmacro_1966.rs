// Generated macro for macro_1966 (macro)
macro_rules! Depcrate_sink_withmacro_1966 {
() => {
// Module: crate::sink::with
// Provides: {"macro_1966"}
// Dependencies: {}
pin_project ! { # [doc = " Sink for the [`with`](super::SinkExt::with) method."] # [must_use = "sinks do nothing unless polled"] pub struct With < Si , Item , U , Fut , F > { # [pin] sink : Si , f : F , # [pin] state : Option < Fut >, _phantom : PhantomData < fn (U) -> Item >, } }
};
}
