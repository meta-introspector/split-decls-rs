macro_rules! emit_def_diagnostic {
    () => {
        fn emit_def_diagnostic < 'db > (db : & 'db dyn HirDatabase , acc : & mut Vec < AnyDiagnostic < 'db > > , diag : & DefDiagnostic , edition : Edition ,) { emit_def_diagnostic_ (db , acc , & diag . kind , edition) }
    };
}

emit_def_diagnostic!()