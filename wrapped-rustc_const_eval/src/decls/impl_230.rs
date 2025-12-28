macro_rules! deps {
    () => {
        MemoryKind!();
        InterpCx!();
        DummyMachine!();
        Memory!();
        PlaceTy!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < 'tcx > InterpCx < 'tcx , DummyMachine > { # [doc = " A helper function that allocates memory for the layout given and gives you access to mutate"] # [doc = " it. Once your own mutation code is done, the backing `Allocation` is removed from the"] # [doc = " current `Memory` and interned as read-only into the global memory."] pub fn intern_with_temp_alloc (& mut self , layout : TyAndLayout < 'tcx > , f : impl FnOnce (& mut InterpCx < 'tcx , DummyMachine > , & PlaceTy < 'tcx , CtfeProvenance > ,) -> InterpResult < 'tcx , () > ,) -> InterpResult < 'tcx , AllocId > { let dest = self . allocate (layout , MemoryKind :: Stack) ? ; f (self , & dest . clone () . into ()) ? ; let alloc_id = dest . ptr () . provenance . unwrap () . alloc_id () ; for prov in intern_shallow (self , alloc_id , Mutability :: Not , None) . unwrap () { if self . tcx . try_get_global_alloc (prov . alloc_id ()) . is_none () { panic ! ("`intern_with_temp_alloc` with nested allocations") ; } } interp_ok (alloc_id) } }
    };
}

impl_230!();