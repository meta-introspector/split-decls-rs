macro_rules! deps {
    () => {
        MemoryKind!();
        CompileTimeInterpCx!();
        MPlaceTy!();
    };
}

macro_rules! create_static_alloc {
    () => {
        deps!();
        pub (crate) fn create_static_alloc < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , static_def_id : LocalDefId , layout : TyAndLayout < 'tcx > ,) -> InterpResult < 'tcx , MPlaceTy < 'tcx > > { let (size , align) = GlobalAlloc :: Static (static_def_id . into ()) . size_and_align (* ecx . tcx , ecx . typing_env) ; assert_eq ! (size , layout . size) ; assert ! (align >= layout . align . abi) ; let alloc = Allocation :: try_new (size , align , AllocInit :: Uninit , ()) ? ; let alloc_id = ecx . tcx . reserve_and_set_static_alloc (static_def_id . into ()) ; assert_eq ! (ecx . machine . static_root_ids , None) ; ecx . machine . static_root_ids = Some ((alloc_id , static_def_id)) ; assert ! (ecx . memory . alloc_map . insert (alloc_id , (MemoryKind :: Stack , alloc)) . is_none ()) ; interp_ok (ecx . ptr_to_mplace (Pointer :: from (alloc_id) . into () , layout)) }
    };
}

create_static_alloc!();