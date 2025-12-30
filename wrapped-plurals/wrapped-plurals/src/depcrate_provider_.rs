// Generated macro for _ (const)
macro_rules! Depcrate_provider_ {
() => {
// Module: crate::provider
// Provides: {"_"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] # [allow (unused_imports)] const _ : () = { use icu_plurals_data :: * ; mod icu { pub use crate as plurals ; pub use icu_locale as locale ; } make_provider ! (Baked) ; impl_plurals_cardinal_v1 ! (Baked) ; impl_plurals_ordinal_v1 ! (Baked) ; # [cfg (feature = "experimental")] impl_plurals_ranges_v1 ! (Baked) ; } ;
};
}
