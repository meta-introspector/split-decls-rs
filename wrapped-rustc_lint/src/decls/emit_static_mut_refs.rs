macro_rules! deps {
    () => {
        RefOfMutStatic!();
        LateContext!();
        MutRefSugg!();
    };
}

macro_rules! emit_static_mut_refs {
    () => {
        deps!();
        fn emit_static_mut_refs (cx : & LateContext < '_ > , span : Span , sugg_span : Span , mutable : Mutability , suggest_addr_of : bool ,) { let (shared_label , shared_note , mut_note , sugg) = match mutable { Mutability :: Mut => { let sugg = if suggest_addr_of { Some (MutRefSugg :: Mut { span : sugg_span }) } else { None } ; ("mutable " , false , true , sugg) } Mutability :: Not => { let sugg = if suggest_addr_of { Some (MutRefSugg :: Shared { span : sugg_span }) } else { None } ; ("shared " , true , false , sugg) } } ; cx . emit_span_lint (STATIC_MUT_REFS , span , RefOfMutStatic { span , sugg , shared_label , shared_note , mut_note } ,) ; }
    };
}

emit_static_mut_refs!();