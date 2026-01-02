mkuse!{use std :: fmt ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_infer :: infer :: region_constraints :: RegionConstraintData ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: { TyCtxt , TypeFoldable } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: info ;}
mkuse!{use crate :: infer :: InferCtxt ;}
mkuse!{use crate :: infer :: canonical :: query_response ;}
mkuse!{use crate :: traits :: ObligationCtxt ;}
mkuse!{use crate :: traits :: query :: type_op :: TypeOpOutput ;}
mkitem!{mkstruct!{pub struct CustomTypeOp < F > { closure : F , description : & 'static str , }}}
mkitem!{mkimpl!{impl < F > CustomTypeOp < F > { pub fn new < 'tcx , R > (closure : F , description : & 'static str) -> Self where F : FnOnce (& ObligationCtxt < '_ , 'tcx >) -> Result < R , NoSolution > , { CustomTypeOp { closure , description } } }}}
mkitem!{mkimpl!{impl < 'tcx , F , R > super :: TypeOp < 'tcx > for CustomTypeOp < F > where F : FnOnce (& ObligationCtxt < '_ , 'tcx >) -> Result < R , NoSolution > , R : fmt :: Debug + TypeFoldable < TyCtxt < 'tcx > > , { type Output = R ; # [doc = " We can't do any custom error reporting for `CustomTypeOp`, so"] # [doc = " we can use `!` to enforce that the implementation never provides it."] type ErrorInfo = ! ; # [doc = " Processes the operation and all resulting obligations,"] # [doc = " returning the final result along with any region constraints"] # [doc = " (they will be given over to the NLL region solver)."] fn fully_perform (self , infcx : & InferCtxt < 'tcx > , root_def_id : LocalDefId , span : Span ,) -> Result < TypeOpOutput < 'tcx , Self > , ErrorGuaranteed > { if cfg ! (debug_assertions) { info ! ("fully_perform({:?})" , self) ; } Ok (scrape_region_constraints (infcx , root_def_id , self . description , span , self . closure) ? . 0) } }}}
mkitem!{mkimpl!{impl < F > fmt :: Debug for CustomTypeOp < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . description . fmt (f) } }}}

macro_rules! scrape_region_constraints_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scrape_region_constraints in module {}", module_path!());
    };
}

mkfn!{
    scrape_region_constraints_introspect!();
    # [doc = " Executes `op` and then scrapes out all the \"old style\" region"] # [doc = " constraints that result, creating query-region-constraints."] pub fn scrape_region_constraints < 'tcx , Op , R > (infcx : & InferCtxt < 'tcx > , root_def_id : LocalDefId , name : & 'static str , span : Span , op : impl FnOnce (& ObligationCtxt < '_ , 'tcx >) -> Result < R , NoSolution > ,) -> Result < (TypeOpOutput < 'tcx , Op > , RegionConstraintData < 'tcx >) , ErrorGuaranteed > where R : TypeFoldable < TyCtxt < 'tcx > > , Op : super :: TypeOp < 'tcx , Output = R > , { let pre_obligations = infcx . take_registered_region_obligations () ; assert ! (pre_obligations . is_empty () , "scrape_region_constraints: incoming region obligations = {pre_obligations:#?}" ,) ; let pre_assumptions = infcx . take_registered_region_assumptions () ; assert ! (pre_assumptions . is_empty () , "scrape_region_constraints: incoming region assumptions = {pre_assumptions:#?}" ,) ; let value = infcx . commit_if_ok (| _ | { let ocx = ObligationCtxt :: new (infcx) ; let value = op (& ocx) . map_err (| _ | { infcx . dcx () . span_delayed_bug (span , format ! ("error performing operation: {name}")) }) ? ; let errors = ocx . select_all_or_error () ; if errors . is_empty () { Ok (value) } else if let Err (guar) = infcx . tcx . check_potentially_region_dependent_goals (root_def_id) { Err (guar) } else { Err (infcx . dcx () . delayed_bug (format ! ("errors selecting obligation during MIR typeck: {errors:?}"))) } }) ? ; let value = infcx . resolve_vars_if_possible (value) ; let region_obligations = infcx . take_registered_region_obligations () ; let region_assumptions = infcx . take_registered_region_assumptions () ; let region_constraint_data = infcx . take_and_reset_region_constraints () ; let region_constraints = query_response :: make_query_region_constraints (region_obligations , & region_constraint_data , region_assumptions ,) ; if region_constraints . is_empty () { Ok ((TypeOpOutput { output : value , constraints : None , error_info : None } , region_constraint_data ,)) } else { Ok ((TypeOpOutput { output : value , constraints : Some (infcx . tcx . arena . alloc (region_constraints)) , error_info : None , } , region_constraint_data ,)) } }
}