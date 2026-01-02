mkuse!{use std :: fmt ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_infer :: traits :: PredicateObligations ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: { ParamEnvAnd , TyCtxt , TypeFoldable } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: infer :: canonical :: { CanonicalQueryInput , CanonicalQueryResponse , Certainty , OriginalQueryValues , QueryRegionConstraints , } ;}
mkuse!{use crate :: infer :: { InferCtxt , InferOk } ;}
mkuse!{use crate :: traits :: { ObligationCause , ObligationCtxt } ;}
mkmod!{ascribe_user_type, { 
                getname!(ascribe_user_type);
                getsrc!(ascribe_user_type);
                getpath!(ascribe_user_type);
                get_deps!(ascribe_user_type);
                get_crates!(ascribe_user_type);
                mkinclude!(ascribe_user_type);
                 
            }}
mkmod!{custom, { 
                getname!(custom);
                getsrc!(custom);
                getpath!(custom);
                get_deps!(custom);
                get_crates!(custom);
                mkinclude!(custom);
                 
            }}
mkmod!{implied_outlives_bounds, { 
                getname!(implied_outlives_bounds);
                getsrc!(implied_outlives_bounds);
                getpath!(implied_outlives_bounds);
                get_deps!(implied_outlives_bounds);
                get_crates!(implied_outlives_bounds);
                mkinclude!(implied_outlives_bounds);
                 
            }}
mkmod!{normalize, { 
                getname!(normalize);
                getsrc!(normalize);
                getpath!(normalize);
                get_deps!(normalize);
                get_crates!(normalize);
                mkinclude!(normalize);
                 
            }}
mkmod!{outlives, { 
                getname!(outlives);
                getsrc!(outlives);
                getpath!(outlives);
                get_deps!(outlives);
                get_crates!(outlives);
                mkinclude!(outlives);
                 
            }}
mkmod!{prove_predicate, { 
                getname!(prove_predicate);
                getsrc!(prove_predicate);
                getpath!(prove_predicate);
                get_deps!(prove_predicate);
                get_crates!(prove_predicate);
                mkinclude!(prove_predicate);
                 
            }}
mkuse!{pub use rustc_middle :: traits :: query :: type_op :: * ;}
mkuse!{use self :: custom :: scrape_region_constraints ;}
mkitem!{mktrait!{# [doc = " \"Type ops\" are used in NLL to perform some particular action and"] # [doc = " extract out the resulting region constraints (or an error if it"] # [doc = " cannot be completed)."] pub trait TypeOp < 'tcx > : Sized + fmt :: Debug { type Output : fmt :: Debug ; type ErrorInfo ; # [doc = " Processes the operation and all resulting obligations,"] # [doc = " returning the final result along with any region constraints"] # [doc = " (they will be given over to the NLL region solver)."] fn fully_perform (self , infcx : & InferCtxt < 'tcx > , root_def_id : LocalDefId , span : Span ,) -> Result < TypeOpOutput < 'tcx , Self > , ErrorGuaranteed > ; }}}
mkitem!{mkstruct!{# [doc = " The output from performing a type op"] pub struct TypeOpOutput < 'tcx , Op : TypeOp < 'tcx > > { # [doc = " The output from the type op."] pub output : Op :: Output , # [doc = " Any region constraints from performing the type op."] pub constraints : Option < & 'tcx QueryRegionConstraints < 'tcx > > , # [doc = " Used for error reporting to be able to rerun the query"] pub error_info : Option < Op :: ErrorInfo > , }}}
mkitem!{mktrait!{# [doc = " \"Query type ops\" are type ops that are implemented using a"] # [doc = " [canonical query][c]. The `Self` type here contains the kernel of"] # [doc = " information needed to do the operation -- `TypeOp` is actually"] # [doc = " implemented for `ParamEnvAnd<Self>`, since we always need to bring"] # [doc = " along a parameter environment as well. For query type-ops, we will"] # [doc = " first canonicalize the key and then invoke the query on the tcx,"] # [doc = " which produces the resulting query region constraints."] # [doc = ""] # [doc = " [c]: https://rust-lang.github.io/chalk/book/canonical_queries/canonicalization.html"] pub trait QueryTypeOp < 'tcx > : fmt :: Debug + Copy + TypeFoldable < TyCtxt < 'tcx > > + 'tcx { type QueryResponse : TypeFoldable < TyCtxt < 'tcx > > ; # [doc = " Give query the option for a simple fast path that never"] # [doc = " actually hits the tcx cache lookup etc. Return `Some(r)` with"] # [doc = " a final result or `None` to do the full path."] fn try_fast_path (tcx : TyCtxt < 'tcx > , key : & ParamEnvAnd < 'tcx , Self > ,) -> Option < Self :: QueryResponse > ; # [doc = " Performs the actual query with the canonicalized key -- the"] # [doc = " real work happens here. This method is not given an `infcx`"] # [doc = " because it shouldn't need one -- and if it had access to one,"] # [doc = " it might do things like invoke `sub_regions`, which would be"] # [doc = " bad, because it would create subregion relationships that are"] # [doc = " not captured in the return value."] fn perform_query (tcx : TyCtxt < 'tcx > , canonicalized : CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Self > > ,) -> Result < CanonicalQueryResponse < 'tcx , Self :: QueryResponse > , NoSolution > ; # [doc = " In the new trait solver, we already do caching in the solver itself,"] # [doc = " so there's no need to canonicalize and cache via the query system."] # [doc = " Additionally, even if we were to canonicalize, we'd still need to"] # [doc = " make sure to feed it predefined opaque types and the defining anchor"] # [doc = " and that would require duplicating all of the tcx queries. Instead,"] # [doc = " just perform these ops locally."] fn perform_locally_with_next_solver (ocx : & ObligationCtxt < '_ , 'tcx > , key : ParamEnvAnd < 'tcx , Self > , span : Span ,) -> Result < Self :: QueryResponse , NoSolution > ; fn fully_perform_into (query_key : ParamEnvAnd < 'tcx , Self > , infcx : & InferCtxt < 'tcx > , output_query_region_constraints : & mut QueryRegionConstraints < 'tcx > , span : Span ,) -> Result < (Self :: QueryResponse , Option < CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Self > > > , PredicateObligations < 'tcx > , Certainty ,) , NoSolution , > { if let Some (result) = QueryTypeOp :: try_fast_path (infcx . tcx , & query_key) { return Ok ((result , None , PredicateObligations :: new () , Certainty :: Proven)) ; } let mut canonical_var_values = OriginalQueryValues :: default () ; let old_param_env = query_key . param_env ; let canonical_self = infcx . canonicalize_query (query_key , & mut canonical_var_values) ; let canonical_result = Self :: perform_query (infcx . tcx , canonical_self) ? ; let InferOk { value , obligations } = infcx . instantiate_nll_query_response_and_region_obligations (& ObligationCause :: dummy_with_span (span) , old_param_env , & canonical_var_values , canonical_result , output_query_region_constraints ,) ? ; Ok ((value , Some (canonical_self) , obligations , canonical_result . value . certainty)) } }}}
mkitem!{mkimpl!{impl < 'tcx , Q > TypeOp < 'tcx > for ParamEnvAnd < 'tcx , Q > where Q : QueryTypeOp < 'tcx > , { type Output = Q :: QueryResponse ; type ErrorInfo = CanonicalQueryInput < 'tcx , ParamEnvAnd < 'tcx , Q > > ; fn fully_perform (self , infcx : & InferCtxt < 'tcx > , root_def_id : LocalDefId , span : Span ,) -> Result < TypeOpOutput < 'tcx , Self > , ErrorGuaranteed > { if infcx . next_trait_solver () { return Ok (scrape_region_constraints (infcx , root_def_id , "query type op" , span , | ocx | QueryTypeOp :: perform_locally_with_next_solver (ocx , self , span) ,) ? . 0) ; } let mut error_info = None ; let mut region_constraints = QueryRegionConstraints :: default () ; let (mut output , _) = scrape_region_constraints (infcx , root_def_id , "fully_perform" , span , | ocx | { let (output , ei , obligations , _) = Q :: fully_perform_into (self , infcx , & mut region_constraints , span) ? ; error_info = ei ; ocx . register_obligations (obligations) ; Ok (output) }) ? ; output . error_info = error_info ; if let Some (QueryRegionConstraints { outlives , assumptions }) = output . constraints { region_constraints . outlives . extend (outlives . iter () . cloned ()) ; region_constraints . assumptions . extend (assumptions . iter () . cloned ()) ; } output . constraints = if region_constraints . is_empty () { None } else { Some (infcx . tcx . arena . alloc (region_constraints)) } ; Ok (output) } }}}