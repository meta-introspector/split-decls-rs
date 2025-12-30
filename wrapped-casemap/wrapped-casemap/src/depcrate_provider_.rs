// Generated macro for _ (const)
macro_rules! Depcrate_provider_ {
() => {
// Module: crate::provider
// Provides: {"_"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] # [allow (unused_imports)] const _ : () = { use icu_casemap_data :: * ; pub mod icu { pub use crate as casemap ; pub use icu_collections as collections ; } make_provider ! (Baked) ; impl_case_map_v1 ! (Baked) ; impl_case_map_unfold_v1 ! (Baked) ; } ;
};
}
