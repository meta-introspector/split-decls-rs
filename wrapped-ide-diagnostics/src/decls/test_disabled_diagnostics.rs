macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! test_disabled_diagnostics {
    () => {
        deps!();
        # [test] fn test_disabled_diagnostics () { let mut config = DiagnosticsConfig :: test_sample () ; config . disabled . insert ("E0583" . into ()) ; let (db , file_id) = RootDatabase :: with_single_file (r#"mod foo;"#) ; let file_id = file_id . file_id (& db) ; let diagnostics = hir :: attach_db (& db , | | { super :: full_diagnostics (& db , & config , & AssistResolveStrategy :: All , file_id) }) ; assert ! (diagnostics . is_empty ()) ; let diagnostics = hir :: attach_db (& db , | | { super :: full_diagnostics (& db , & DiagnosticsConfig :: test_sample () , & AssistResolveStrategy :: All , file_id ,) }) ; assert ! (! diagnostics . is_empty ()) ; }
    };
}

test_disabled_diagnostics!()