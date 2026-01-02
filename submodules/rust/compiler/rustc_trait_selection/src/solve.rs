mkuse!{pub use rustc_next_trait_solver :: solve :: * ;}
mkmod!{delegate, { 
                getname!(delegate);
                getsrc!(delegate);
                getpath!(delegate);
                get_deps!(delegate);
                get_crates!(delegate);
                mkinclude!(delegate);
                 
            }}
mkmod!{fulfill, { 
                getname!(fulfill);
                getsrc!(fulfill);
                getpath!(fulfill);
                get_deps!(fulfill);
                get_crates!(fulfill);
                mkinclude!(fulfill);
                 
            }}
mkmod!{inspect, { 
                getname!(inspect);
                getsrc!(inspect);
                getpath!(inspect);
                get_deps!(inspect);
                get_crates!(inspect);
                mkinclude!(inspect);
                 
            }}
mkmod!{normalize, { 
                getname!(normalize);
                getsrc!(normalize);
                getpath!(normalize);
                get_deps!(normalize);
                get_crates!(normalize);
                mkinclude!(normalize);
                 
            }}
mkmod!{select, { 
                getname!(select);
                getsrc!(select);
                getpath!(select);
                get_deps!(select);
                get_crates!(select);
                mkinclude!(select);
                 
            }}
mkuse!{pub (crate) use delegate :: SolverDelegate ;}
mkuse!{pub use fulfill :: { FulfillmentCtxt , NextSolverError , StalledOnCoroutines } ;}
mkuse!{pub (crate) use normalize :: deeply_normalize_for_diagnostics ;}
mkuse!{pub use normalize :: { deeply_normalize , deeply_normalize_with_skipped_universes , deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals , } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{pub use select :: InferCtxtSelectExt ;}

macro_rules! evaluate_root_goal_for_proof_tree_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_root_goal_for_proof_tree_raw in module {}", module_path!());
    };
}

mkfn!{
    evaluate_root_goal_for_proof_tree_raw_introspect!();
    fn evaluate_root_goal_for_proof_tree_raw < 'tcx > (tcx : TyCtxt < 'tcx > , canonical_input : CanonicalInput < TyCtxt < 'tcx > > ,) -> (QueryResult < TyCtxt < 'tcx > > , & 'tcx inspect :: Probe < TyCtxt < 'tcx > >) { evaluate_root_goal_for_proof_tree_raw_provider :: < SolverDelegate < 'tcx > , TyCtxt < 'tcx > > (tcx , canonical_input ,) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { * providers = Providers { evaluate_root_goal_for_proof_tree_raw , .. * providers } ; }
}