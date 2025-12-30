// Generated macro for State (enum)
macro_rules! Depcrate_store_impls_dynamic_iterState {
() => {
// Module: crate::store_impls::dynamic::iter
// Provides: {"State"}
// Dependencies: {}
enum State { Pack { index_iter : IntoIter < handle :: IndexLookup > , index : handle :: IndexLookup , ordered_entries : Option < Vec < EntryForOrdering > > , entry_index : u32 , num_objects : u32 , } , Loose { iter : loose :: Iter , index : usize , } , Depleted , }
};
}
