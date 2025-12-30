// Generated macro for aggregated_table_properties_at_level (function)
macro_rules! Depcrate_propertiesaggregated_table_properties_at_level {
() => {
// Module: crate::properties
// Provides: {"aggregated_table_properties_at_level"}
// Dependencies: {}
# [doc = " \"rocksdb.aggregated-table-properties-at-`level<N>`\", same as the previous"] # [doc = " one but only returns the aggregated table properties of the"] # [doc = " specified level \"N\" at the target column family."] pub fn aggregated_table_properties_at_level (level : usize) -> PropertyName { unsafe { level_property ("aggregated-table-properties-at-level" , level) } }
};
}
