mkuse!{use rustc_infer :: infer :: InferOk ;}
mkuse!{use rustc_infer :: infer :: resolve :: OpportunisticRegionResolver ;}
mkuse!{use rustc_infer :: traits :: query :: type_op :: ImpliedOutlivesBounds ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{use rustc_middle :: infer :: canonical :: { OriginalQueryValues , QueryRegionConstraints } ;}
mkuse!{pub use rustc_middle :: traits :: query :: OutlivesBound ;}
mkuse!{use rustc_middle :: ty :: { self , ParamEnv , Ty , TypeFolder , TypeVisitableExt } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: infer :: InferCtxt ;}
mkuse!{use crate :: traits :: ObligationCause ;}

macro_rules! implied_outlives_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function implied_outlives_bounds in module {}", module_path!());
    };
}

mkfn!{
    implied_outlives_bounds_introspect!();
    # [doc = " Implied bounds are region relationships that we deduce"] # [doc = " automatically. The idea is that (e.g.) a caller must check that a"] # [doc = " function's argument types are well-formed immediately before"] # [doc = " calling that fn, and hence the *callee* can assume that its"] # [doc = " argument types are well-formed. This may imply certain relationships"] # [doc = " between generic parameters. For example:"] # [doc = " ```"] # [doc = " fn foo<T>(x: &T) {}"] # [doc = " ```"] # [doc = " can only be called with a `'a` and `T` such that `&'a T` is WF."] # [doc = " For `&'a T` to be WF, `T: 'a` must hold. So we can assume `T: 'a`."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `param_env`, the where-clauses in scope"] # [doc = " - `body_id`, the body-id to use when normalizing assoc types."] # [doc = "   Note that this may cause outlives obligations to be injected"] # [doc = "   into the inference context with this body-id."] # [doc = " - `ty`, the type that we are supposed to assume is WF."] # [instrument (level = "debug" , skip (infcx , param_env , body_id) , ret)] fn implied_outlives_bounds < 'a , 'tcx > (infcx : & 'a InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , body_id : LocalDefId , ty : Ty < 'tcx > , disable_implied_bounds_hack : bool ,) -> Vec < OutlivesBound < 'tcx > > { let ty = infcx . resolve_vars_if_possible (ty) ; let ty = OpportunisticRegionResolver :: new (infcx) . fold_ty (ty) ; assert ! (! ty . has_non_region_infer ()) ; let mut canonical_var_values = OriginalQueryValues :: default () ; let input = ImpliedOutlivesBounds { ty } ; let canonical = infcx . canonicalize_query (param_env . and (input) , & mut canonical_var_values) ; let implied_bounds_result = infcx . tcx . implied_outlives_bounds ((canonical , disable_implied_bounds_hack)) ; let Ok (canonical_result) = implied_bounds_result else { return vec ! [] ; } ; let mut constraints = QueryRegionConstraints :: default () ; let span = infcx . tcx . def_span (body_id) ; let Ok (InferOk { value : mut bounds , obligations }) = infcx . instantiate_nll_query_response_and_region_obligations (& ObligationCause :: dummy_with_span (span) , param_env , & canonical_var_values , canonical_result , & mut constraints ,) else { return vec ! [] ; } ; assert_eq ! (obligations . len () , 0) ; bounds . retain (| bound | ! bound . has_placeholders ()) ; if ! constraints . is_empty () { let QueryRegionConstraints { outlives , assumptions : _ } = constraints ; let cause = ObligationCause :: misc (span , body_id) ; for & (predicate , _) in & outlives { infcx . register_outlives_constraint (predicate , & cause) ; } } ; bounds }
}
mkitem!{mkimpl!{# [extension (pub trait InferCtxtExt <'tcx >)] impl < 'tcx > InferCtxt < 'tcx > { # [doc = " Do *NOT* call this directly. You probably want to construct a `OutlivesEnvironment`"] # [doc = " instead if you're interested in the implied bounds for a given signature."] fn implied_bounds_tys < Tys : IntoIterator < Item = Ty < 'tcx > > > (& self , body_id : LocalDefId , param_env : ParamEnv < 'tcx > , tys : Tys , disable_implied_bounds_hack : bool ,) -> impl Iterator < Item = OutlivesBound < 'tcx > > { tys . into_iter () . flat_map (move | ty | { implied_outlives_bounds (self , param_env , body_id , ty , disable_implied_bounds_hack) }) } }}}