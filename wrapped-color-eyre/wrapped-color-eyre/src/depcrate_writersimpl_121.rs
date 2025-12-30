// Generated macro for impl_121 (impl)
macro_rules! Depcrate_writersimpl_121 {
() => {
// Module: crate::writers
// Provides: {"impl_121"}
// Dependencies: {}
# [cfg (feature = "issue-url")] impl < T > DisplayExt for T where T : Display , { fn with_footer < F : Display > (self , footer : F) -> Footer < Self , F > { Footer { body : self , footer } } fn with_header < H : Display > (self , header : H) -> Header < Self , H > { Header { body : self , h : header , } } }
};
}
