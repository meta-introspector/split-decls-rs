// Generated macro for _ (const)
macro_rules! Depcrate_provider_ {
() => {
// Module: crate::provider
// Provides: {"_"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] # [allow (unused_imports)] const _ : () = { use icu_calendar_data :: * ; pub mod icu { pub use crate as calendar ; pub use icu_locale as locale ; } make_provider ! (Baked) ; impl_calendar_japanese_modern_v1 ! (Baked) ; impl_calendar_japanese_extended_v1 ! (Baked) ; impl_calendar_week_v1 ! (Baked) ; } ;
};
}
