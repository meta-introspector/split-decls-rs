// Generated macro for fetch_crates (function)
macro_rules! Depcrate_fetch_cratesfetch_crates {
() => {
// Module: crate::fetch_crates
// Provides: {"fetch_crates"}
// Dependencies: {}
pub (crate) fn fetch_crates (db : & RootDatabase) -> FxIndexSet < CrateInfo > { db . all_crates () . iter () . copied () . map (| crate_id | (crate_id . data (db) , crate_id . extra_data (db))) . filter (| (data , _) | ! matches ! (data . origin , CrateOrigin :: Local { .. })) . map (| (data , extra_data) | crate_info (data , extra_data)) . collect () }
};
}
