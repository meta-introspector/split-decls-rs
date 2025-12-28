macro_rules! UpvarMigrationInfo {
    () => {
        # [doc = " Intermediate format to store the hir_id pointing to the use that resulted in the"] # [doc = " corresponding place being captured and a String which contains the captured value's"] # [doc = " name (i.e: a.b.c)"] # [derive (Clone , Debug , PartialEq , Eq , Hash)] enum UpvarMigrationInfo { # [doc = " We previously captured all of `x`, but now we capture some sub-path."] CapturingPrecise { source_expr : Option < HirId > , var_name : String } , CapturingNothing { use_span : Span , } , }
    };
}

UpvarMigrationInfo!();