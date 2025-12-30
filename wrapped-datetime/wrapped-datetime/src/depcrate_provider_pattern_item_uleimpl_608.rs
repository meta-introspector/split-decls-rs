// Generated macro for impl_608 (impl)
macro_rules! Depcrate_provider_pattern_item_uleimpl_608 {
() => {
// Module: crate::provider::pattern::item::ule
// Provides: {"impl_608"}
// Dependencies: {}
unsafe impl ULE for PatternItemULE { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . len () % 3 != 0 { return Err (UleError :: length :: < Self > (bytes . len ())) ; } # [expect (clippy :: indexing_slicing)] if ! bytes . chunks (3) . all (| c | Self :: bytes_in_range ((& c [0] , & c [1] , & c [2]))) { return Err (UleError :: parse :: < Self > ()) ; } Ok (()) } }
};
}
