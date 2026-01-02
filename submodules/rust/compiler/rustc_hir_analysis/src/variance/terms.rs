mkuse!{use std :: fmt ;}
mkuse!{use rustc_arena :: DroplessArena ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { LocalDefId , LocalDefIdMap } ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use self :: VarianceTerm :: * ;}
mkitem!{pub (crate) type VarianceTermPtr < 'a > = & 'a VarianceTerm < 'a > ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug)] pub (crate) struct InferredIndex (pub usize) ;}}
mkitem!{mkenum!{# [derive (Copy , Clone)] pub (crate) enum VarianceTerm < 'a > { ConstantTerm (ty :: Variance) , TransformTerm (VarianceTermPtr < 'a > , VarianceTermPtr < 'a >) , InferredTerm (InferredIndex) , }}}
mkitem!{mkimpl!{impl < 'a > fmt :: Debug for VarianceTerm < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ConstantTerm (c1) => write ! (f , "{c1:?}") , TransformTerm (v1 , v2) => write ! (f , "({v1:?} \u{00D7} {v2:?})") , InferredTerm (id) => write ! (f , "[{}]" , { let InferredIndex (i) = id ; i }) , } } }}}
mkitem!{mkstruct!{# [doc = " The first pass over the crate simply builds up the set of inferreds."] pub (crate) struct TermsContext < 'a , 'tcx > { pub tcx : TyCtxt < 'tcx > , pub arena : & 'a DroplessArena , # [doc = " For marker types, `UnsafeCell`, and other lang items where"] # [doc = " variance is hardcoded, records the item-id and the hardcoded"] # [doc = " variance."] pub lang_items : Vec < (LocalDefId , Vec < ty :: Variance >) > , # [doc = " Maps from the node id of an item to the first inferred index"] # [doc = " used for its type & region parameters."] pub inferred_starts : LocalDefIdMap < InferredIndex > , # [doc = " Maps from an InferredIndex to the term for that variable."] pub inferred_terms : Vec < VarianceTermPtr < 'a > > , }}}

macro_rules! determine_parameters_to_be_inferred_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function determine_parameters_to_be_inferred in module {}", module_path!());
    };
}

mkfn!{
    determine_parameters_to_be_inferred_introspect!();
    pub (crate) fn determine_parameters_to_be_inferred < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , arena : & 'a DroplessArena ,) -> TermsContext < 'a , 'tcx > { let mut terms_cx = TermsContext { tcx , arena , inferred_starts : Default :: default () , inferred_terms : vec ! [] , lang_items : lang_items (tcx) , } ; let crate_items = tcx . hir_crate_items (()) ; for def_id in crate_items . definitions () { debug ! ("add_inferreds for item {:?}" , def_id) ; let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: Struct | DefKind :: Union | DefKind :: Enum => { terms_cx . add_inferreds_for_item (def_id) ; let adt = tcx . adt_def (def_id) ; for variant in adt . variants () { if let Some (ctor_def_id) = variant . ctor_def_id () { terms_cx . add_inferreds_for_item (ctor_def_id . expect_local ()) ; } } } DefKind :: Fn | DefKind :: AssocFn => terms_cx . add_inferreds_for_item (def_id) , DefKind :: TyAlias if tcx . type_alias_is_lazy (def_id) => { terms_cx . add_inferreds_for_item (def_id) } _ => { } } } terms_cx }
}

macro_rules! lang_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lang_items in module {}", module_path!());
    };
}

mkfn!{
    lang_items_introspect!();
    fn lang_items (tcx : TyCtxt < '_ >) -> Vec < (LocalDefId , Vec < ty :: Variance >) > { let lang_items = tcx . lang_items () ; let all = [(lang_items . phantom_data () , vec ! [ty :: Covariant]) , (lang_items . unsafe_cell_type () , vec ! [ty :: Invariant]) ,] ; all . into_iter () . filter_map (| (d , v) | { let def_id = d ? . as_local () ? ; Some ((def_id , v)) }) . collect () }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > TermsContext < 'a , 'tcx > { fn add_inferreds_for_item (& mut self , def_id : LocalDefId) { let tcx = self . tcx ; let count = tcx . generics_of (def_id) . count () ; if count == 0 { return ; } let start = self . inferred_terms . len () ; let newly_added = self . inferred_starts . insert (def_id , InferredIndex (start)) . is_none () ; assert ! (newly_added) ; let arena = self . arena ; self . inferred_terms . extend ((start .. (start + count)) . map (| i | & * arena . alloc (InferredTerm (InferredIndex (i)))) ,) ; } }}}