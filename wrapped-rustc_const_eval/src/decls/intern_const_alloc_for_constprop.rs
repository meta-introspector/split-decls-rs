macro_rules! deps {
    () => {
        InterpCx!();
        CompileTimeMachine!();
    };
}

macro_rules! intern_const_alloc_for_constprop {
    () => {
        deps!();
        # [doc = " Intern `ret`. This function assumes that `ret` references no other allocation."] # [instrument (level = "debug" , skip (ecx))] pub fn intern_const_alloc_for_constprop < 'tcx , M : CompileTimeMachine < 'tcx > > (ecx : & mut InterpCx < 'tcx , M > , alloc_id : AllocId ,) -> InterpResult < 'tcx , () > { if ecx . tcx . try_get_global_alloc (alloc_id) . is_some () { return interp_ok (()) ; } if let Some (_) = intern_shallow (ecx , alloc_id , Mutability :: Not , None) . unwrap () . next () { panic ! ("`intern_const_alloc_for_constprop` called on allocation with nested provenance") } interp_ok (()) }
    };
}

intern_const_alloc_for_constprop!();