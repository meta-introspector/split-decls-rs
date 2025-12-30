// Generated macro for CollectedQuery (struct)
macro_rules! Depcrate_query_builder_collected_queryCollectedQuery {
() => {
// Module: crate::query_builder::collected_query
// Provides: {"CollectedQuery"}
// Dependencies: {}
# [derive (Debug)] # [must_use = "Queries are only executed when calling `load`, `get_result` or similar."] # [doc = " A SQL query variant with already collected bind data which can be moved"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub struct CollectedQuery < T > { sql : String , safe_to_cache_prepared : bool , bind_data : T , }
};
}
