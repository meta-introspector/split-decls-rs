macro_rules! deps {
    () => {
        InternError!();
        InternKind!();
        MPlaceTy!();
        InterpCx!();
        CompileTimeMachine!();
    };
}

macro_rules! intern_const_alloc_recursive {
    () => {
        deps!();
        # [doc = " Intern `ret` and everything it references."] # [doc = ""] # [doc = " This *cannot raise an interpreter error*. Doing so is left to validation, which"] # [doc = " tracks where in the value we are and thus can show much better error messages."] # [doc = ""] # [doc = " For `InternKind::Static` the root allocation will not be interned, but must be handled by the caller."] # [instrument (level = "debug" , skip (ecx))] pub fn intern_const_alloc_recursive < 'tcx , M : CompileTimeMachine < 'tcx > > (ecx : & mut InterpCx < 'tcx , M > , intern_kind : InternKind , ret : & MPlaceTy < 'tcx > ,) -> Result < () , InternError > { let mut disambiguator = DisambiguatorState :: new () ; let (base_mutability , inner_mutability , is_static) = match intern_kind { InternKind :: Constant | InternKind :: Promoted => { (Mutability :: Not , Mutability :: Not , false) } InternKind :: Static (Mutability :: Not) => { (if ret . layout . ty . is_freeze (* ecx . tcx , ecx . typing_env) { Mutability :: Not } else { Mutability :: Mut } , Mutability :: Not , true ,) } InternKind :: Static (Mutability :: Mut) => { (Mutability :: Mut , Mutability :: Mut , true) } } ; let base_alloc_id = ret . ptr () . provenance . unwrap () . alloc_id () ; trace ! (? base_alloc_id , ? base_mutability) ; let mut todo : Vec < _ > = if is_static { let (kind , alloc) = ecx . memory . alloc_map . get_mut (& base_alloc_id) . unwrap () ; prepare_alloc (* ecx . tcx , * kind , alloc , base_mutability) ? ; alloc . provenance () . ptrs () . iter () . map (| & (_ , prov) | prov) . collect () } else { intern_shallow (ecx , base_alloc_id , base_mutability , Some (& mut disambiguator)) ? . collect () } ; let mut just_interned : FxHashSet < _ > = std :: iter :: once (base_alloc_id) . collect () ; let mut found_bad_mutable_ptr = false ; while let Some (prov) = todo . pop () { trace ! (? prov) ; let alloc_id = prov . alloc_id () ; if base_alloc_id == alloc_id && is_static { continue ; } if intern_kind != InternKind :: Promoted && inner_mutability == Mutability :: Not && ! prov . shared_ref () { let is_already_global = ecx . tcx . try_get_global_alloc (alloc_id) . is_some () ; if is_already_global && ! just_interned . contains (& alloc_id) { continue ; } let dangling = ! is_already_global && ! ecx . memory . alloc_map . contains_key (& alloc_id) ; if ! dangling { found_bad_mutable_ptr = true ; } } if ecx . tcx . try_get_global_alloc (alloc_id) . is_some () { debug_assert ! (! ecx . memory . alloc_map . contains_key (& alloc_id)) ; continue ; } just_interned . insert (alloc_id) ; let next = intern_shallow (ecx , alloc_id , inner_mutability , Some (& mut disambiguator)) ? ; todo . extend (next) ; } if found_bad_mutable_ptr { if ecx . tcx . sess . opts . unstable_opts . unleash_the_miri_inside_of_you { return Err (InternError :: BadMutablePointer) ; } else { span_bug ! (ecx . tcx . span , "the static const safety checks accepted a mutable pointer they should not have accepted") ; } } Ok (()) }
    };
}

intern_const_alloc_recursive!()