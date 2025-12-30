// Generated macro for _ (const)
macro_rules! Depcrate_provider_ {
() => {
// Module: crate::provider
// Provides: {"_"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] # [allow (unused_imports)] const _ : () = { use icu_decimal_data :: * ; pub mod icu { pub use crate as decimal ; pub use icu_locale as locale ; } make_provider ! (Baked) ; impl_decimal_symbols_v1 ! (Baked) ; impl_decimal_digits_v1 ! (Baked) ; } ;
};
}
