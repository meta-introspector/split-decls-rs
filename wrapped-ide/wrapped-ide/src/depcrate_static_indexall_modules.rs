// Generated macro for all_modules (function)
macro_rules! Depcrate_static_indexall_modules {
() => {
// Module: crate::static_index
// Provides: {"all_modules"}
// Dependencies: {}
fn all_modules (db : & dyn HirDatabase) -> Vec < Module > { let mut worklist : Vec < _ > = Crate :: all (db) . into_iter () . map (| krate | krate . root_module ()) . collect () ; let mut modules = Vec :: new () ; while let Some (module) = worklist . pop () { modules . push (module) ; worklist . extend (module . children (db)) ; } modules }
};
}
