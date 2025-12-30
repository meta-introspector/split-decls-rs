// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " A prelude for using the datagen API"] pub mod prelude { # [doc (no_inline)] pub use crate :: { DataLocaleFamily , DeduplicationStrategy , ExportDriver , FallbackOptions , NoFallbackOptions , } ; # [doc (no_inline)] pub use icu_locale :: { locale , LocaleFallbacker } ; # [doc (no_inline)] pub use icu_provider :: { export :: DataExporter , DataLocale , DataMarker , DataMarkerInfo } ; }
};
}
