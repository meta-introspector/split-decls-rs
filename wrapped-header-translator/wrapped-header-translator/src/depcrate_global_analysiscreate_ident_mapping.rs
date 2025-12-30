// Generated macro for create_ident_mapping (function)
macro_rules! Depcrate_global_analysiscreate_ident_mapping {
() => {
// Module: crate::global_analysis
// Provides: {"create_ident_mapping"}
// Dependencies: {}
fn create_ident_mapping (module : & Module) -> HashMap < String , Expr > { let mut mapping = HashMap :: new () ; for stmt in & module . stmts { mapping . extend (stmt . get_ident_mapping ()) ; } for submodule in module . submodules . values () { mapping . extend (create_ident_mapping (submodule)) ; } mapping }
};
}
