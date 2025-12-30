// Generated macro for EnvSection (struct)
macro_rules! Depcrate_writersEnvSection {
() => {
// Module: crate::writers
// Provides: {"EnvSection"}
// Dependencies: {}
pub (crate) struct EnvSection < 'a > { pub (crate) bt_captured : & 'a bool , # [cfg (feature = "capture-spantrace")] pub (crate) span_trace : Option < & 'a SpanTrace > , }
};
}
