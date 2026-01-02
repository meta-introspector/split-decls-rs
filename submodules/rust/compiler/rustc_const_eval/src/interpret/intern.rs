mkuse!{use hir :: def :: DefKind ;}
mkuse!{use rustc_ast :: Mutability ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: definitions :: { DefPathData , DisambiguatorState } ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrs ;}
mkuse!{use rustc_middle :: mir :: interpret :: { AllocBytes , ConstAllocation , CtfeProvenance , InterpResult , Provenance , } ;}
mkuse!{use rustc_middle :: query :: TyCtxtAt ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_middle :: ty :: layout :: TyAndLayout ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use tracing :: { instrument , trace } ;}
mkuse!{use super :: { AllocId , Allocation , InterpCx , MPlaceTy , Machine , MemoryKind , PlaceTy , interp_ok } ;}
mkuse!{use crate :: const_eval :: DummyMachine ;}
mkuse!{use crate :: { const_eval , errors } ;}
mkitem!{pub trait CompileTimeMachine < 'tcx > = Machine < 'tcx , MemoryKind = const_eval :: MemoryKind , Provenance = CtfeProvenance , ExtraFnVal = ! , FrameExtra = () , AllocExtra = () , MemoryMap = FxIndexMap < AllocId , (MemoryKind < const_eval :: MemoryKind > , Allocation) > , > + HasStaticRootDefId ;}
mkitem!{mktrait!{pub trait HasStaticRootDefId { # [doc = " Returns the `DefId` of the static item that is currently being evaluated."] # [doc = " Used for interning to be able to handle nested allocations."] fn static_def_id (& self) -> Option < LocalDefId > ; }}}
mkitem!{mkimpl!{impl HasStaticRootDefId for const_eval :: CompileTimeMachine < '_ > { fn static_def_id (& self) -> Option < LocalDefId > { Some (self . static_root_ids ? . 1) } }}}

macro_rules! prepare_alloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_alloc in module {}", module_path!());
    };
}

mkfn!{
    prepare_alloc_introspect!();
    fn prepare_alloc < 'tcx , Prov : Provenance , Extra , Bytes : AllocBytes > (tcx : TyCtxt < 'tcx > , kind : MemoryKind < const_eval :: MemoryKind > , alloc : & mut Allocation < Prov , Extra , Bytes > , mutability : Mutability ,) -> Result < () , InternError > { match kind { MemoryKind :: Machine (const_eval :: MemoryKind :: Heap { was_made_global }) => { if ! was_made_global { tcx . dcx () . delayed_bug ("non-global heap allocation in const value") ; return Err (InternError :: ConstAllocNotGlobal) ; } } MemoryKind :: Stack | MemoryKind :: CallerLocation => { } } if ! alloc . provenance_merge_bytes (& tcx) { tcx . dcx () . delayed_bug ("partial pointer in const value") ; return Err (InternError :: PartialPointer) ; } match mutability { Mutability :: Not => { alloc . mutability = Mutability :: Not ; } Mutability :: Mut => { assert_eq ! (alloc . mutability , Mutability :: Mut) ; } } Ok (()) }
}

macro_rules! intern_shallow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function intern_shallow in module {}", module_path!());
    };
}

mkfn!{
    intern_shallow_introspect!();
    # [doc = " Intern an allocation. Returns `Err` if the allocation does not exist in the local memory."] # [doc = ""] # [doc = " `mutability` can be used to force immutable interning: if it is `Mutability::Not`, the"] # [doc = " allocation is interned immutably; if it is `Mutability::Mut`, then the allocation *must be*"] # [doc = " already mutable (as a sanity check)."] # [doc = ""] # [doc = " Returns an iterator over all relocations referred to by this allocation."] fn intern_shallow < 'tcx , M : CompileTimeMachine < 'tcx > > (ecx : & mut InterpCx < 'tcx , M > , alloc_id : AllocId , mutability : Mutability , disambiguator : Option < & mut DisambiguatorState > ,) -> Result < impl Iterator < Item = CtfeProvenance > + 'tcx , InternError > { trace ! ("intern_shallow {:?}" , alloc_id) ; let Some ((kind , mut alloc)) = ecx . memory . alloc_map . swap_remove (& alloc_id) else { return Err (InternError :: DanglingPointer) ; } ; if let Err (err) = prepare_alloc (* ecx . tcx , kind , & mut alloc , mutability) { ecx . memory . alloc_map . insert (alloc_id , (kind , alloc)) ; return Err (err) ; } let alloc = ecx . tcx . mk_const_alloc (alloc) ; if let Some (static_id) = ecx . machine . static_def_id () { intern_as_new_static (ecx . tcx , static_id , alloc_id , alloc , disambiguator . expect ("disambiguator needed") ,) ; } else { ecx . tcx . set_alloc_id_memory (alloc_id , alloc) ; } Ok (alloc . inner () . provenance () . ptrs () . iter () . map (| & (_ , prov) | prov)) }
}

macro_rules! intern_as_new_static_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function intern_as_new_static in module {}", module_path!());
    };
}

mkfn!{
    intern_as_new_static_introspect!();
    # [doc = " Creates a new `DefId` and feeds all the right queries to make this `DefId`"] # [doc = " appear as if it were a user-written `static` (though it has no HIR)."] fn intern_as_new_static < 'tcx > (tcx : TyCtxtAt < 'tcx > , static_id : LocalDefId , alloc_id : AllocId , alloc : ConstAllocation < 'tcx > , disambiguator : & mut DisambiguatorState ,) { let feed = tcx . create_def (static_id , None , DefKind :: Static { safety : hir :: Safety :: Safe , mutability : alloc . 0 . mutability , nested : true } , Some (DefPathData :: NestedStatic) , disambiguator ,) ; tcx . set_nested_alloc_id_static (alloc_id , feed . def_id ()) ; if tcx . is_thread_local_static (static_id . into ()) { tcx . dcx () . emit_err (errors :: NestedStaticInThreadLocal { span : tcx . def_span (static_id) }) ; } feed . codegen_fn_attrs (CodegenFnAttrs :: new ()) ; feed . eval_static_initializer (Ok (alloc)) ; feed . generics_of (tcx . generics_of (static_id) . clone ()) ; feed . def_ident_span (tcx . def_ident_span (static_id)) ; feed . explicit_predicates_of (tcx . explicit_predicates_of (static_id)) ; feed . feed_hir () ; }
}
mkitem!{mkenum!{# [doc = " How a constant value should be interned."] # [derive (Copy , Clone , Debug , PartialEq , Hash , Eq)] pub enum InternKind { # [doc = " The `mutability` of the static, ignoring the type which may have interior mutability."] Static (hir :: Mutability) , # [doc = " A `const` item"] Constant , Promoted , }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum InternError { BadMutablePointer , DanglingPointer , ConstAllocNotGlobal , PartialPointer , }}}

macro_rules! intern_const_alloc_recursive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function intern_const_alloc_recursive in module {}", module_path!());
    };
}

mkfn!{
    intern_const_alloc_recursive_introspect!();
    # [doc = " Intern `ret` and everything it references."] # [doc = ""] # [doc = " This *cannot raise an interpreter error*. Doing so is left to validation, which"] # [doc = " tracks where in the value we are and thus can show much better error messages."] # [doc = ""] # [doc = " For `InternKind::Static` the root allocation will not be interned, but must be handled by the caller."] # [instrument (level = "debug" , skip (ecx))] pub fn intern_const_alloc_recursive < 'tcx , M : CompileTimeMachine < 'tcx > > (ecx : & mut InterpCx < 'tcx , M > , intern_kind : InternKind , ret : & MPlaceTy < 'tcx > ,) -> Result < () , InternError > { let mut disambiguator = DisambiguatorState :: new () ; let (base_mutability , inner_mutability , is_static) = match intern_kind { InternKind :: Constant | InternKind :: Promoted => { (Mutability :: Not , Mutability :: Not , false) } InternKind :: Static (Mutability :: Not) => { (if ret . layout . ty . is_freeze (* ecx . tcx , ecx . typing_env) { Mutability :: Not } else { Mutability :: Mut } , Mutability :: Not , true ,) } InternKind :: Static (Mutability :: Mut) => { (Mutability :: Mut , Mutability :: Mut , true) } } ; let base_alloc_id = ret . ptr () . provenance . unwrap () . alloc_id () ; trace ! (? base_alloc_id , ? base_mutability) ; let mut todo : Vec < _ > = if is_static { let (kind , alloc) = ecx . memory . alloc_map . get_mut (& base_alloc_id) . unwrap () ; prepare_alloc (* ecx . tcx , * kind , alloc , base_mutability) ? ; alloc . provenance () . ptrs () . iter () . map (| & (_ , prov) | prov) . collect () } else { intern_shallow (ecx , base_alloc_id , base_mutability , Some (& mut disambiguator)) ? . collect () } ; let mut just_interned : FxHashSet < _ > = std :: iter :: once (base_alloc_id) . collect () ; let mut found_bad_mutable_ptr = false ; while let Some (prov) = todo . pop () { trace ! (? prov) ; let alloc_id = prov . alloc_id () ; if base_alloc_id == alloc_id && is_static { continue ; } if intern_kind != InternKind :: Promoted && inner_mutability == Mutability :: Not && ! prov . shared_ref () { let is_already_global = ecx . tcx . try_get_global_alloc (alloc_id) . is_some () ; if is_already_global && ! just_interned . contains (& alloc_id) { continue ; } let dangling = ! is_already_global && ! ecx . memory . alloc_map . contains_key (& alloc_id) ; if ! dangling { found_bad_mutable_ptr = true ; } } if ecx . tcx . try_get_global_alloc (alloc_id) . is_some () { debug_assert ! (! ecx . memory . alloc_map . contains_key (& alloc_id)) ; continue ; } just_interned . insert (alloc_id) ; let next = intern_shallow (ecx , alloc_id , inner_mutability , Some (& mut disambiguator)) ? ; todo . extend (next) ; } if found_bad_mutable_ptr { if ecx . tcx . sess . opts . unstable_opts . unleash_the_miri_inside_of_you { return Err (InternError :: BadMutablePointer) ; } else { span_bug ! (ecx . tcx . span , "the static const safety checks accepted a mutable pointer they should not have accepted") ; } } Ok (()) }
}

macro_rules! intern_const_alloc_for_constprop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function intern_const_alloc_for_constprop in module {}", module_path!());
    };
}

mkfn!{
    intern_const_alloc_for_constprop_introspect!();
    # [doc = " Intern `ret`. This function assumes that `ret` references no other allocation."] # [instrument (level = "debug" , skip (ecx))] pub fn intern_const_alloc_for_constprop < 'tcx , M : CompileTimeMachine < 'tcx > > (ecx : & mut InterpCx < 'tcx , M > , alloc_id : AllocId ,) -> InterpResult < 'tcx , () > { if ecx . tcx . try_get_global_alloc (alloc_id) . is_some () { return interp_ok (()) ; } if let Some (_) = intern_shallow (ecx , alloc_id , Mutability :: Not , None) . unwrap () . next () { panic ! ("`intern_const_alloc_for_constprop` called on allocation with nested provenance") } interp_ok (()) }
}
mkitem!{mkimpl!{impl < 'tcx > InterpCx < 'tcx , DummyMachine > { # [doc = " A helper function that allocates memory for the layout given and gives you access to mutate"] # [doc = " it. Once your own mutation code is done, the backing `Allocation` is removed from the"] # [doc = " current `Memory` and interned as read-only into the global memory."] pub fn intern_with_temp_alloc (& mut self , layout : TyAndLayout < 'tcx > , f : impl FnOnce (& mut InterpCx < 'tcx , DummyMachine > , & PlaceTy < 'tcx , CtfeProvenance > ,) -> InterpResult < 'tcx , () > ,) -> InterpResult < 'tcx , AllocId > { let dest = self . allocate (layout , MemoryKind :: Stack) ? ; f (self , & dest . clone () . into ()) ? ; let alloc_id = dest . ptr () . provenance . unwrap () . alloc_id () ; for prov in intern_shallow (self , alloc_id , Mutability :: Not , None) . unwrap () { if self . tcx . try_get_global_alloc (prov . alloc_id ()) . is_none () { panic ! ("`intern_with_temp_alloc` with nested allocations") ; } } interp_ok (alloc_id) } }}}