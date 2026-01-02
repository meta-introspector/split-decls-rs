mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_infer :: traits :: util :: PredicateSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , GenericParamDefKind , Ty , TyCtxt , TypeVisitableExt , Upcast , VtblEntry , } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: traits :: { impossible_predicates , is_vtable_safe_method } ;}
mkitem!{mkenum!{# [derive (Clone , Debug)] pub enum VtblSegment < 'tcx > { MetadataDSA , TraitOwnEntries { trait_ref : ty :: TraitRef < 'tcx > , emit_vptr : bool } , }}}

macro_rules! prepare_vtable_segments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_vtable_segments in module {}", module_path!());
    };
}

mkfn!{
    prepare_vtable_segments_introspect!();
    # [doc = " Prepare the segments for a vtable"] pub fn prepare_vtable_segments < 'tcx , T > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > , segment_visitor : impl FnMut (VtblSegment < 'tcx >) -> ControlFlow < T > ,) -> Option < T > { prepare_vtable_segments_inner (tcx , trait_ref , segment_visitor) . break_value () }
}

macro_rules! prepare_vtable_segments_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_vtable_segments_inner in module {}", module_path!());
    };
}

mkfn!{
    prepare_vtable_segments_inner_introspect!();
    # [doc = " Helper for [`prepare_vtable_segments`] that returns `ControlFlow`,"] # [doc = " such that we can use `?` in the body."] fn prepare_vtable_segments_inner < 'tcx , T > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > , mut segment_visitor : impl FnMut (VtblSegment < 'tcx >) -> ControlFlow < T > ,) -> ControlFlow < T > { segment_visitor (VtblSegment :: MetadataDSA) ? ; let mut emit_vptr_on_new_entry = false ; let mut visited = PredicateSet :: new (tcx) ; let predicate = trait_ref . upcast (tcx) ; let mut stack : SmallVec < [(ty :: TraitRef < 'tcx > , _ , _) ; 5] > = smallvec ! [(trait_ref , emit_vptr_on_new_entry , maybe_iter (None))] ; visited . insert (predicate) ; 'outer : loop { 'diving_in : loop { let & (inner_most_trait_ref , _ , _) = stack . last () . unwrap () ; let mut direct_super_traits_iter = tcx . explicit_super_predicates_of (inner_most_trait_ref . def_id) . iter_identity_copied () . filter_map (move | (pred , _) | { pred . instantiate_supertrait (tcx , ty :: Binder :: dummy (inner_most_trait_ref)) . as_trait_clause () }) . map (move | pred | { tcx . normalize_erasing_late_bound_regions (ty :: TypingEnv :: fully_monomorphized () , pred ,) . trait_ref }) ; match direct_super_traits_iter . find (| & super_trait | visited . insert (super_trait . upcast (tcx))) { Some (next_super_trait) => stack . push ((next_super_trait , emit_vptr_on_new_entry , maybe_iter (Some (direct_super_traits_iter)) ,)) , None => break 'diving_in , } } while let Some ((inner_most_trait_ref , emit_vptr , mut siblings)) = stack . pop () { let has_entries = ty :: elaborate :: supertrait_def_ids (tcx , inner_most_trait_ref . def_id) . any (| def_id | has_own_existential_vtable_entries (tcx , def_id)) ; segment_visitor (VtblSegment :: TraitOwnEntries { trait_ref : inner_most_trait_ref , emit_vptr : emit_vptr && has_entries && ! tcx . sess . opts . unstable_opts . no_trait_vptr , }) ? ; emit_vptr_on_new_entry |= has_entries ; if let Some (next_inner_most_trait_ref) = siblings . find (| & sibling | visited . insert (sibling . upcast (tcx))) { stack . push ((next_inner_most_trait_ref , emit_vptr_on_new_entry , siblings)) ; continue 'outer ; } } return ControlFlow :: Continue (()) ; } }
}

macro_rules! maybe_iter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_iter in module {}", module_path!());
    };
}

mkfn!{
    maybe_iter_introspect!();
    # [doc = " Turns option of iterator into an iterator (this is just flatten)"] fn maybe_iter < I : Iterator > (i : Option < I >) -> impl Iterator < Item = I :: Item > { i . into_iter () . flatten () }
}

macro_rules! has_own_existential_vtable_entries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_own_existential_vtable_entries in module {}", module_path!());
    };
}

mkfn!{
    has_own_existential_vtable_entries_introspect!();
    fn has_own_existential_vtable_entries (tcx : TyCtxt < '_ > , trait_def_id : DefId) -> bool { own_existential_vtable_entries_iter (tcx , trait_def_id) . next () . is_some () }
}

macro_rules! own_existential_vtable_entries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function own_existential_vtable_entries in module {}", module_path!());
    };
}

mkfn!{
    own_existential_vtable_entries_introspect!();
    fn own_existential_vtable_entries (tcx : TyCtxt < '_ > , trait_def_id : DefId) -> & [DefId] { tcx . arena . alloc_from_iter (own_existential_vtable_entries_iter (tcx , trait_def_id)) }
}

macro_rules! own_existential_vtable_entries_iter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function own_existential_vtable_entries_iter in module {}", module_path!());
    };
}

mkfn!{
    own_existential_vtable_entries_iter_introspect!();
    fn own_existential_vtable_entries_iter (tcx : TyCtxt < '_ > , trait_def_id : DefId ,) -> impl Iterator < Item = DefId > { let trait_methods = tcx . associated_items (trait_def_id) . in_definition_order () . filter (| item | item . is_fn ()) ; let own_entries = trait_methods . filter_map (move | & trait_method | { debug ! ("own_existential_vtable_entry: trait_method={:?}" , trait_method) ; let def_id = trait_method . def_id ; if ! is_vtable_safe_method (tcx , trait_def_id , trait_method) { debug ! ("own_existential_vtable_entry: not vtable safe") ; return None ; } Some (def_id) }) ; own_entries }
}

macro_rules! vtable_entries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vtable_entries in module {}", module_path!());
    };
}

mkfn!{
    vtable_entries_introspect!();
    # [doc = " Given a trait `trait_ref`, iterates the vtable entries"] # [doc = " that come from `trait_ref`, including its supertraits."] fn vtable_entries < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > ,) -> & 'tcx [VtblEntry < 'tcx >] { debug_assert ! (! trait_ref . has_non_region_infer () && ! trait_ref . has_non_region_param ()) ; debug_assert_eq ! (tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , trait_ref) , trait_ref , "vtable trait ref should be normalized") ; debug ! ("vtable_entries({:?})" , trait_ref) ; let mut entries = vec ! [] ; let vtable_segment_callback = | segment | -> ControlFlow < () > { match segment { VtblSegment :: MetadataDSA => { entries . extend (TyCtxt :: COMMON_VTABLE_ENTRIES) ; } VtblSegment :: TraitOwnEntries { trait_ref , emit_vptr } => { let existential_trait_ref = ty :: ExistentialTraitRef :: erase_self_ty (tcx , trait_ref) ; let own_existential_entries = tcx . own_existential_vtable_entries (existential_trait_ref . def_id) ; let own_entries = own_existential_entries . iter () . copied () . map (| def_id | { debug ! ("vtable_entries: trait_method={:?}" , def_id) ; let args = tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , GenericArgs :: for_item (tcx , def_id , | param , _ | match param . kind { GenericParamDefKind :: Lifetime => tcx . lifetimes . re_erased . into () , GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { trait_ref . args [param . index as usize] } }) ,) ; let predicates = tcx . predicates_of (def_id) . instantiate_own (tcx , args) ; if impossible_predicates (tcx , predicates . map (| (predicate , _) | predicate) . collect () ,) { debug ! ("vtable_entries: predicates do not hold") ; return VtblEntry :: Vacant ; } let instance = ty :: Instance :: expect_resolve_for_vtable (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args , DUMMY_SP ,) ; VtblEntry :: Method (instance) }) ; entries . extend (own_entries) ; if emit_vptr { entries . push (VtblEntry :: TraitVPtr (trait_ref)) ; } } } ControlFlow :: Continue (()) } ; let _ = prepare_vtable_segments (tcx , trait_ref , vtable_segment_callback) ; tcx . arena . alloc_from_iter (entries) }
}

macro_rules! first_method_vtable_slot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function first_method_vtable_slot in module {}", module_path!());
    };
}

mkfn!{
    first_method_vtable_slot_introspect!();
    pub (crate) fn first_method_vtable_slot < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: TraitRef < 'tcx >) -> usize { debug_assert ! (! key . has_non_region_infer () && ! key . has_non_region_param ()) ; debug_assert_eq ! (tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , key) , key , "vtable trait ref should be normalized") ; let ty :: Dynamic (source , _ , _) = * key . self_ty () . kind () else { bug ! () ; } ; let source_principal = tcx . instantiate_bound_regions_with_erased (source . principal () . unwrap () . with_self_ty (tcx , key . self_ty ()) ,) ; if tcx . instantiate_and_check_impossible_predicates ((source_principal . def_id , source_principal . args ,)) { return 0 ; } let target_principal = ty :: ExistentialTraitRef :: erase_self_ty (tcx , key) ; let vtable_segment_callback = { let mut vptr_offset = 0 ; move | segment | { match segment { VtblSegment :: MetadataDSA => { vptr_offset += TyCtxt :: COMMON_VTABLE_ENTRIES . len () ; } VtblSegment :: TraitOwnEntries { trait_ref : vtable_principal , emit_vptr } => { if ty :: ExistentialTraitRef :: erase_self_ty (tcx , vtable_principal) == target_principal { return ControlFlow :: Break (vptr_offset) ; } vptr_offset += tcx . own_existential_vtable_entries (vtable_principal . def_id) . len () ; if emit_vptr { vptr_offset += 1 ; } } } ControlFlow :: Continue (()) } } ; prepare_vtable_segments (tcx , source_principal , vtable_segment_callback) . unwrap () }
}

macro_rules! supertrait_vtable_slot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function supertrait_vtable_slot in module {}", module_path!());
    };
}

mkfn!{
    supertrait_vtable_slot_introspect!();
    # [doc = " Given a `dyn Subtrait` and `dyn Supertrait` trait object, find the slot of"] # [doc = " the trait vptr in the subtrait's vtable."] # [doc = ""] # [doc = " A return value of `None` means that the original vtable can be reused."] pub (crate) fn supertrait_vtable_slot < 'tcx > (tcx : TyCtxt < 'tcx > , key : (Ty < 'tcx > , Ty < 'tcx > ,) ,) -> Option < usize > { debug_assert ! (! key . has_non_region_infer () && ! key . has_non_region_param ()) ; debug_assert_eq ! (tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , key) , key , "upcasting trait refs should be normalized") ; let (source , target) = key ; let ty :: Dynamic (target_data , _ , _) = * target . kind () else { bug ! () ; } ; let target_principal = tcx . instantiate_bound_regions_with_erased (target_data . principal () ?) ; let ty :: Dynamic (source_data , _ , _) = * source . kind () else { bug ! () ; } ; let source_principal = tcx . instantiate_bound_regions_with_erased (source_data . principal () . unwrap () . with_self_ty (tcx , source) ,) ; if tcx . instantiate_and_check_impossible_predicates ((source_principal . def_id , source_principal . args ,)) { return None ; } let vtable_segment_callback = { let mut vptr_offset = 0 ; move | segment | { match segment { VtblSegment :: MetadataDSA => { vptr_offset += TyCtxt :: COMMON_VTABLE_ENTRIES . len () ; } VtblSegment :: TraitOwnEntries { trait_ref : vtable_principal , emit_vptr } => { vptr_offset += tcx . own_existential_vtable_entries (vtable_principal . def_id) . len () ; if ty :: ExistentialTraitRef :: erase_self_ty (tcx , vtable_principal) == target_principal { if emit_vptr { return ControlFlow :: Break (Some (vptr_offset)) ; } else { return ControlFlow :: Break (None) ; } } if emit_vptr { vptr_offset += 1 ; } } } ControlFlow :: Continue (()) } } ; prepare_vtable_segments (tcx , source_principal , vtable_segment_callback) . unwrap () }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (super) fn provide (providers : & mut Providers) { * providers = Providers { own_existential_vtable_entries , vtable_entries , first_method_vtable_slot , supertrait_vtable_slot , .. * providers } ; }
}