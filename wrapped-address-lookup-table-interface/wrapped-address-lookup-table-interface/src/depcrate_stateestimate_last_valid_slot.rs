// Generated macro for estimate_last_valid_slot (function)
macro_rules! Depcrate_stateestimate_last_valid_slot {
() => {
// Module: crate::state
// Provides: {"estimate_last_valid_slot"}
// Dependencies: {}
# [doc = " The lookup table may be in a deactivating state until"] # [doc = " the `deactivation_slot`` is no longer \"recent\"."] # [doc = " This function returns a conservative estimate for the"] # [doc = " last block that the table may be used for lookups."] # [doc = " This estimate may be incorrect due to skipped blocks,"] # [doc = " however, if the current slot is lower than the returned"] # [doc = " value, the table is guaranteed to still be in the"] # [doc = " deactivating state."] # [inline] pub fn estimate_last_valid_slot (deactivation_slot : Slot) -> Slot { deactivation_slot . saturating_add (get_entries () as Slot) }
};
}
