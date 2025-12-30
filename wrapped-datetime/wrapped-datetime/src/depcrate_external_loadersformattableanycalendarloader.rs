// Generated macro for FormattableAnyCalendarLoader (trait)
macro_rules! Depcrate_external_loadersFormattableAnyCalendarLoader {
() => {
// Module: crate::external_loaders
// Provides: {"FormattableAnyCalendarLoader"}
// Dependencies: {}
# [doc = " Trait for loading an AnyCalendar."] # [doc = ""] # [doc = " Implemented on the provider-specific loader types in this module."] pub (crate) trait FormattableAnyCalendarLoader { fn load (& self , kind : FormattableAnyCalendarKind) -> Result < FormattableAnyCalendar , DataError > ; }
};
}
