macro_rules! deps {
    () => {
        ClashingExternDeclarations!();
    };
}

macro_rules! clashing_extern_declarations {
    () => {
        deps!();
        fn clashing_extern_declarations (tcx : TyCtxt < '_ > , () : ()) { let mut lint = ClashingExternDeclarations :: new () ; for id in tcx . hir_crate_items (()) . foreign_items () { lint . check_foreign_item (tcx , id) ; } }
    };
}

clashing_extern_declarations!()