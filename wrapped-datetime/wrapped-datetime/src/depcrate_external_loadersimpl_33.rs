// Generated macro for impl_33 (impl)
macro_rules! Depcrate_external_loadersimpl_33 {
() => {
// Module: crate::external_loaders
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < P > FormattableAnyCalendarLoader for ExternalLoaderBuffer < '_ , P > where P : ? Sized + BufferProvider , { # [inline] fn load (& self , kind : FormattableAnyCalendarKind) -> Result < FormattableAnyCalendar , DataError > { FormattableAnyCalendar :: try_new_with_buffer_provider (self . 0 , kind) } }
};
}
