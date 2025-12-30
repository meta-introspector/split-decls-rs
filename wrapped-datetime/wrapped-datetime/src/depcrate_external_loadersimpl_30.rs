// Generated macro for impl_30 (impl)
macro_rules! Depcrate_external_loadersimpl_30 {
() => {
// Module: crate::external_loaders
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] impl FormattableAnyCalendarLoader for ExternalLoaderCompiledData { # [inline] fn load (& self , kind : FormattableAnyCalendarKind) -> Result < FormattableAnyCalendar , DataError > { FormattableAnyCalendar :: try_new (kind) } }
};
}
