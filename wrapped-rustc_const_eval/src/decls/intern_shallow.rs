macro_rules! deps {
    () => {
        CompileTimeMachine!();
        InterpCx!();
        InternError!();
    };
}

macro_rules! intern_shallow {
    () => {
        deps!();
        # [doc = " Intern an allocation. Returns `Err` if the allocation does not exist in the local memory."] # [doc = ""] # [doc = " `mutability` can be used to force immutable interning: if it is `Mutability::Not`, the"] # [doc = " allocation is interned immutably; if it is `Mutability::Mut`, then the allocation *must be*"] # [doc = " already mutable (as a sanity check)."] # [doc = ""] # [doc = " Returns an iterator over all relocations referred to by this allocation."] fn intern_shallow < 'tcx , M : CompileTimeMachine < 'tcx > > (ecx : & mut InterpCx < 'tcx , M > , alloc_id : AllocId , mutability : Mutability , disambiguator : Option < & mut DisambiguatorState > ,) -> Result < impl Iterator < Item = CtfeProvenance > + 'tcx , InternError > { trace ! ("intern_shallow {:?}" , alloc_id) ; let Some ((kind , mut alloc)) = ecx . memory . alloc_map . swap_remove (& alloc_id) else { return Err (InternError :: DanglingPointer) ; } ; if let Err (err) = prepare_alloc (* ecx . tcx , kind , & mut alloc , mutability) { ecx . memory . alloc_map . insert (alloc_id , (kind , alloc)) ; return Err (err) ; } let alloc = ecx . tcx . mk_const_alloc (alloc) ; if let Some (static_id) = ecx . machine . static_def_id () { intern_as_new_static (ecx . tcx , static_id , alloc_id , alloc , disambiguator . expect ("disambiguator needed") ,) ; } else { ecx . tcx . set_alloc_id_memory (alloc_id , alloc) ; } Ok (alloc . inner () . provenance () . ptrs () . iter () . map (| & (_ , prov) | prov)) }
    };
}

intern_shallow!();