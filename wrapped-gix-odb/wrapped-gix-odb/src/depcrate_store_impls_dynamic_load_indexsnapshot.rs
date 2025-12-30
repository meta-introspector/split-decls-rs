// Generated macro for Snapshot (struct)
macro_rules! Depcrate_store_impls_dynamic_load_indexSnapshot {
() => {
// Module: crate::store_impls::dynamic::load_index
// Provides: {"Snapshot"}
// Dependencies: {}
pub (crate) struct Snapshot { # [doc = " Indices ready for object lookup or contains checks, ordered usually by modification data, recent ones first."] pub (crate) indices : Vec < handle :: IndexLookup > , # [doc = " A set of loose objects dbs to search once packed objects weren't found."] pub (crate) loose_dbs : Arc < Vec < crate :: loose :: Store > > , # [doc = " remember what this state represents and to compare to other states."] pub (crate) marker : types :: SlotIndexMarker , }
};
}
