mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_infer :: infer :: canonical :: { self , Canonical } ;}
mkuse!{use rustc_infer :: traits :: query :: OutlivesBound ;}
mkuse!{use rustc_infer :: traits :: query :: type_op :: ImpliedOutlivesBounds ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { ParamEnvAnd , TyCtxt } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use rustc_trait_selection :: infer :: InferCtxtBuilderExt ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: implied_outlives_bounds :: compute_implied_outlives_bounds_inner ;}
mkuse!{use rustc_trait_selection :: traits :: query :: { CanonicalImpliedOutlivesBoundsGoal , NoSolution } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (p : & mut Providers) { * p = Providers { implied_outlives_bounds , .. * p } ; }
}

macro_rules! implied_outlives_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function implied_outlives_bounds in module {}", module_path!());
    };
}

mkfn!{
    implied_outlives_bounds_introspect!();
    fn implied_outlives_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , (goal , disable_implied_bounds_hack) : (CanonicalImpliedOutlivesBoundsGoal < 'tcx > , bool) ,) -> Result < & 'tcx Canonical < 'tcx , canonical :: QueryResponse < 'tcx , Vec < OutlivesBound < 'tcx > > > > , NoSolution , > { tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , key | { let ParamEnvAnd { param_env , value : ImpliedOutlivesBounds { ty } } = key ; compute_implied_outlives_bounds_inner (ocx , param_env , ty , DUMMY_SP , disable_implied_bounds_hack ,) }) }
}