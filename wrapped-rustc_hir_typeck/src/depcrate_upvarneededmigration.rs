// Generated macro for NeededMigration (struct)
macro_rules! Depcrate_upvarNeededMigration {
() => {
// Module: crate::upvar
// Provides: {"NeededMigration"}
// Dependencies: {}
# [doc = " Intermediate format to store the hir id of the root variable and a HashSet containing"] # [doc = " information on why the root variable should be fully captured"] struct NeededMigration { var_hir_id : HirId , diagnostics_info : Vec < MigrationLintNote > , }
};
}
