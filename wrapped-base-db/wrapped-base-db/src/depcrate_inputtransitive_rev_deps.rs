// Generated macro for transitive_rev_deps (function)
macro_rules! Depcrate_inputtransitive_rev_deps {
() => {
// Module: crate::input
// Provides: {"transitive_rev_deps"}
// Dependencies: {}
pub (crate) fn transitive_rev_deps (db : & dyn RootQueryDb , of : Crate) -> FxHashSet < Crate > { let mut worklist = vec ! [of] ; let mut rev_deps = FxHashSet :: default () ; rev_deps . insert (of) ; let mut inverted_graph = FxHashMap :: < _ , Vec < _ > > :: default () ; db . all_crates () . iter () . for_each (| & krate | { krate . data (db) . dependencies . iter () . for_each (| dep | inverted_graph . entry (dep . crate_id) . or_default () . push (krate)) }) ; while let Some (krate) = worklist . pop () { if let Some (crate_rev_deps) = inverted_graph . get (& krate) { crate_rev_deps . iter () . copied () . filter (| & rev_dep | rev_deps . insert (rev_dep)) . for_each (| rev_dep | worklist . push (rev_dep)) ; } } rev_deps }
};
}
