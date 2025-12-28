macro_rules! deps {
    () => {
        CompileTimeInterpCx!();
        MPlaceTy!();
        MemoryKind!();
    };
}

macro_rules! create_valtree_place {
    () => {
        deps!();
        # [instrument (skip (ecx) , level = "debug" , ret)] fn create_valtree_place < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , layout : TyAndLayout < 'tcx > , valtree : ty :: ValTree < 'tcx > ,) -> MPlaceTy < 'tcx > { let meta = reconstruct_place_meta (layout , valtree , ecx . tcx . tcx) ; ecx . allocate_dyn (layout , MemoryKind :: Stack , meta) . unwrap () }
    };
}

create_valtree_place!();