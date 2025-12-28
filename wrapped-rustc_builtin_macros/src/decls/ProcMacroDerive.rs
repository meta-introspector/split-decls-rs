macro_rules! ProcMacroDerive {
    () => {
        struct ProcMacroDerive { id : NodeId , trait_name : Symbol , function_ident : Ident , span : Span , attrs : ThinVec < Symbol > , }
    };
}

ProcMacroDerive!();