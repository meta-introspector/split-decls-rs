macro_rules! ProcMacroDef {
    () => {
        struct ProcMacroDef { id : NodeId , function_ident : Ident , span : Span , }
    };
}

ProcMacroDef!();