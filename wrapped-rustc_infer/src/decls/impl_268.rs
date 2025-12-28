macro_rules! deps {
    () => {
        InferCtxt!();
        InferCtxtInner!();
        InferCtxtBuilder!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < 'tcx > InferCtxtBuilder < 'tcx > { pub fn with_next_trait_solver (mut self , next_trait_solver : bool) -> Self { self . next_trait_solver = next_trait_solver ; self } pub fn ignoring_regions (mut self) -> Self { self . considering_regions = false ; self } pub fn in_hir_typeck (mut self) -> Self { self . in_hir_typeck = true ; self } pub fn skip_leak_check (mut self , skip_leak_check : bool) -> Self { self . skip_leak_check = skip_leak_check ; self } # [doc = " Given a canonical value `C` as a starting point, create an"] # [doc = " inference context that contains each of the bound values"] # [doc = " within instantiated as a fresh variable. The `f` closure is"] # [doc = " invoked with the new infcx, along with the instantiated value"] # [doc = " `V` and a instantiation `S`. This instantiation `S` maps from"] # [doc = " the bound values in `C` to their instantiated values in `V`"] # [doc = " (in other words, `S(C) = V`)."] pub fn build_with_canonical < T > (mut self , span : Span , input : & CanonicalQueryInput < 'tcx , T > ,) -> (InferCtxt < 'tcx > , T , CanonicalVarValues < 'tcx >) where T : TypeFoldable < TyCtxt < 'tcx > > , { let infcx = self . build (input . typing_mode) ; let (value , args) = infcx . instantiate_canonical (span , & input . canonical) ; (infcx , value , args) } pub fn build_with_typing_env (mut self , TypingEnv { typing_mode , param_env } : TypingEnv < 'tcx > ,) -> (InferCtxt < 'tcx > , ty :: ParamEnv < 'tcx >) { (self . build (typing_mode) , param_env) } pub fn build (& mut self , typing_mode : TypingMode < 'tcx >) -> InferCtxt < 'tcx > { let InferCtxtBuilder { tcx , considering_regions , in_hir_typeck , skip_leak_check , next_trait_solver , } = * self ; InferCtxt { tcx , typing_mode , considering_regions , in_hir_typeck , skip_leak_check , inner : RefCell :: new (InferCtxtInner :: new ()) , lexical_region_resolutions : RefCell :: new (None) , selection_cache : Default :: default () , evaluation_cache : Default :: default () , reported_trait_errors : Default :: default () , reported_signature_mismatch : Default :: default () , tainted_by_errors : Cell :: new (None) , universe : Cell :: new (ty :: UniverseIndex :: ROOT) , next_trait_solver , obligation_inspector : Cell :: new (None) , } } }
    };
}

impl_268!();