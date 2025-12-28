macro_rules! deps {
    () => {
        NeededMigration!();
    };
}

macro_rules! migration_suggestion_for_2229 {
    () => {
        deps!();
        # [doc = " Return a two string tuple (s1, s2)"] # [doc = " - s1: Line of code that is needed for the migration: eg: `let _ = (&x, ...)`."] # [doc = " - s2: Comma separated names of the variables being migrated."] fn migration_suggestion_for_2229 (tcx : TyCtxt < '_ > , need_migrations : & [NeededMigration] ,) -> (String , String) { let need_migrations_variables = need_migrations . iter () . map (| NeededMigration { var_hir_id : v , .. } | var_name (tcx , * v)) . collect :: < Vec < _ > > () ; let migration_ref_concat = need_migrations_variables . iter () . map (| v | format ! ("&{v}")) . collect :: < Vec < _ > > () . join (", ") ; let migration_string = if 1 == need_migrations . len () { format ! ("let _ = {migration_ref_concat}") } else { format ! ("let _ = ({migration_ref_concat})") } ; let migrated_variables_concat = need_migrations_variables . iter () . map (| v | format ! ("`{v}`")) . collect :: < Vec < _ > > () . join (", ") ; (migration_string , migrated_variables_concat) }
    };
}

migration_suggestion_for_2229!();