// Generated macro for create_implementable_mapping (function)
macro_rules! Depcrate_global_analysiscreate_implementable_mapping {
() => {
// Module: crate::global_analysis
// Provides: {"create_implementable_mapping"}
// Dependencies: {}
fn create_implementable_mapping (module : & Module) -> BTreeSet < ItemTree > { let mut types = BTreeSet :: new () ; for stmt in & module . stmts { if stmt . implementable () { types . insert (ItemTree :: new (stmt . provided_item () . unwrap () , stmt . required_items () ,)) ; } } for submodule in module . submodules . values () { types . extend (create_implementable_mapping (submodule)) ; } types }
};
}
