mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_infer :: infer :: canonical :: { Canonical , QueryResponse } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: traits :: query :: { DropckConstraint , DropckOutlivesResult } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , TyCtxt } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtBuilderExt ;}
mkuse!{use rustc_trait_selection :: traits :: query :: dropck_outlives :: { compute_dropck_outlives_inner , dtorck_constraint_for_ty_inner , } ;}
mkuse!{use rustc_trait_selection :: traits :: query :: { CanonicalDropckOutlivesGoal , NoSolution } ;}
mkuse!{use tracing :: debug ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (p : & mut Providers) { * p = Providers { dropck_outlives , adt_dtorck_constraint , .. * p } ; }
}

macro_rules! dropck_outlives_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dropck_outlives in module {}", module_path!());
    };
}

mkfn!{
    dropck_outlives_introspect!();
    fn dropck_outlives < 'tcx > (tcx : TyCtxt < 'tcx > , canonical_goal : CanonicalDropckOutlivesGoal < 'tcx > ,) -> Result < & 'tcx Canonical < 'tcx , QueryResponse < 'tcx , DropckOutlivesResult < 'tcx > > > , NoSolution > { debug ! ("dropck_outlives(goal={:#?})" , canonical_goal) ; tcx . infer_ctxt () . enter_canonical_trait_query (& canonical_goal , | ocx , goal | { compute_dropck_outlives_inner (ocx , goal , DUMMY_SP) }) }
}

macro_rules! adt_dtorck_constraint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_dtorck_constraint in module {}", module_path!());
    };
}

mkfn!{
    adt_dtorck_constraint_introspect!();
    # [doc = " Calculates the dtorck constraint for a type."] pub (crate) fn adt_dtorck_constraint (tcx : TyCtxt < '_ > , def_id : DefId) -> & DropckConstraint < '_ > { let def = tcx . adt_def (def_id) ; let span = tcx . def_span (def_id) ; let typing_env = ty :: TypingEnv :: non_body_analysis (tcx , def_id) ; debug ! ("dtorck_constraint: {:?}" , def) ; if def . is_manually_drop () { bug ! ("`ManuallyDrop` should have been handled by `trivial_dropck_outlives`") ; } else if def . is_phantom_data () { let args = GenericArgs :: identity_for_item (tcx , def_id) ; assert_eq ! (args . len () , 1) ; let result = DropckConstraint { outlives : vec ! [] , dtorck_types : vec ! [args . type_at (0)] , overflows : vec ! [] , } ; debug ! ("dtorck_constraint: {:?} => {:?}" , def , result) ; return tcx . arena . alloc (result) ; } let mut result = DropckConstraint :: empty () ; for field in def . all_fields () { let fty = tcx . type_of (field . did) . instantiate_identity () ; dtorck_constraint_for_ty_inner (tcx , typing_env , span , 0 , fty , & mut result) ; } result . outlives . extend (tcx . destructor_constraints (def)) ; dedup_dtorck_constraint (& mut result) ; debug ! ("dtorck_constraint: {:?} => {:?}" , def , result) ; tcx . arena . alloc (result) }
}

macro_rules! dedup_dtorck_constraint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dedup_dtorck_constraint in module {}", module_path!());
    };
}

mkfn!{
    dedup_dtorck_constraint_introspect!();
    fn dedup_dtorck_constraint (c : & mut DropckConstraint < '_ >) { let mut outlives = FxHashSet :: default () ; let mut dtorck_types = FxHashSet :: default () ; c . outlives . retain (| & val | outlives . replace (val) . is_none ()) ; c . dtorck_types . retain (| & val | dtorck_types . replace (val) . is_none ()) ; }
}