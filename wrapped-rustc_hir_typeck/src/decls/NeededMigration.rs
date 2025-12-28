macro_rules! deps {
    () => {
        MigrationLintNote!();
    };
}

macro_rules! NeededMigration {
    () => {
        deps!();
        # [doc = " Intermediate format to store the hir id of the root variable and a HashSet containing"] # [doc = " information on why the root variable should be fully captured"] struct NeededMigration { var_hir_id : HirId , diagnostics_info : Vec < MigrationLintNote > , }
    };
}

NeededMigration!();