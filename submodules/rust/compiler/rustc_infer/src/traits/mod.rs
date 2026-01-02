mkmod!{engine, { 
                getname!(engine);
                getsrc!(engine);
                getpath!(engine);
                get_deps!(engine);
                get_crates!(engine);
                mkinclude!(engine);
                 
            }}
mkmod!{project, { 
                getname!(project);
                getsrc!(project);
                getpath!(project);
                get_deps!(project);
                get_crates!(project);
                mkinclude!(project);
                 
            }}
mkmod!{structural_impls, { 
                getname!(structural_impls);
                getsrc!(structural_impls);
                getpath!(structural_impls);
                get_deps!(structural_impls);
                get_crates!(structural_impls);
                mkinclude!(structural_impls);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkuse!{use std :: cmp ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_macros :: { TypeFoldable , TypeVisitable } ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: traits :: solve :: Certainty ;}
mkuse!{pub use rustc_middle :: traits :: * ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , Upcast } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{pub use self :: engine :: { FromSolverError , ScrubbedTraitError , TraitEngine } ;}
mkuse!{pub (crate) use self :: project :: UndoLog ;}
mkuse!{pub use self :: project :: { MismatchedProjectionTypes , Normalized , NormalizedTerm , ProjectionCache , ProjectionCacheEntry , ProjectionCacheKey , ProjectionCacheStorage , } ;}
mkuse!{use crate :: infer :: InferCtxt ;}
mkitem!{mkstruct!{# [doc = " An `Obligation` represents some trait reference (e.g., `i32: Eq`) for"] # [doc = " which the \"impl_source\" must be found. The process of finding an \"impl_source\" is"] # [doc = " called \"resolving\" the `Obligation`. This process consists of"] # [doc = " either identifying an `impl` (e.g., `impl Eq for i32`) that"] # [doc = " satisfies the obligation, or else finding a bound that is in"] # [doc = " scope. The eventual result is usually a `Selection` (defined below)."] # [derive (Clone , TypeFoldable , TypeVisitable)] pub struct Obligation < 'tcx , T > { # [doc = " The reason we have to prove this thing."] # [type_foldable (identity)] # [type_visitable (ignore)] pub cause : ObligationCause < 'tcx > , # [doc = " The environment in which we should prove this thing."] pub param_env : ty :: ParamEnv < 'tcx > , # [doc = " The thing we are trying to prove."] pub predicate : T , # [doc = " If we started proving this as a result of trying to prove"] # [doc = " something else, track the total depth to ensure termination."] # [doc = " If this goes over a certain threshold, we abort compilation --"] # [doc = " in such cases, we can not say whether or not the predicate"] # [doc = " holds for certain. Stupid halting problem; such a drag."] # [type_foldable (identity)] # [type_visitable (ignore)] pub recursion_depth : usize , }}}
mkitem!{mkimpl!{impl < 'tcx , T : Copy > Obligation < 'tcx , T > { pub fn as_goal (& self) -> solve :: Goal < 'tcx , T > { solve :: Goal { param_env : self . param_env , predicate : self . predicate } } }}}
mkitem!{mkimpl!{impl < 'tcx , T : PartialEq > PartialEq < Obligation < 'tcx , T > > for Obligation < 'tcx , T > { # [inline] fn eq (& self , other : & Obligation < 'tcx , T >) -> bool { self . param_env == other . param_env && self . predicate == other . predicate } }}}
mkitem!{mkimpl!{impl < T : Eq > Eq for Obligation < '_ , T > { }}}
mkitem!{mkimpl!{impl < T : Hash > Hash for Obligation < '_ , T > { fn hash < H : Hasher > (& self , state : & mut H) -> () { self . param_env . hash (state) ; self . predicate . hash (state) ; } }}}
mkitem!{pub type PredicateObligation < 'tcx > = Obligation < 'tcx , ty :: Predicate < 'tcx > > ;}
mkitem!{pub type TraitObligation < 'tcx > = Obligation < 'tcx , ty :: TraitPredicate < 'tcx > > ;}
mkitem!{pub type PolyTraitObligation < 'tcx > = Obligation < 'tcx , ty :: PolyTraitPredicate < 'tcx > > ;}
mkitem!{pub type PredicateObligations < 'tcx > = ThinVec < PredicateObligation < 'tcx > > ;}
mkitem!{mkimpl!{impl < 'tcx > PredicateObligation < 'tcx > { # [doc = " Flips the polarity of the inner predicate."] # [doc = ""] # [doc = " Given `T: Trait` predicate it returns `T: !Trait` and given `T: !Trait` returns `T: Trait`."] pub fn flip_polarity (& self , tcx : TyCtxt < 'tcx >) -> Option < PredicateObligation < 'tcx > > { Some (PredicateObligation { cause : self . cause . clone () , param_env : self . param_env , predicate : self . predicate . flip_polarity (tcx) ? , recursion_depth : self . recursion_depth , }) } }}}
mkitem!{mkimpl!{impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn derived_cause (& self , variant : impl FnOnce (DerivedCause < 'tcx >) -> ObligationCauseCode < 'tcx > ,) -> ObligationCause < 'tcx > { self . cause . clone () . derived_cause (self . predicate , variant) } }}}
mkitem!{# [cfg (target_pointer_width = "64")] rustc_data_structures :: static_assert_size ! (PredicateObligation <'_ >, 48) ;}
mkitem!{pub type Selection < 'tcx > = ImplSource < 'tcx , PredicateObligation < 'tcx > > ;}
mkitem!{# [doc = " A callback that can be provided to `inspect_typeck`. Invoked on evaluation"] # [doc = " of root obligations."] pub type ObligationInspector < 'tcx > = fn (& InferCtxt < 'tcx > , & PredicateObligation < 'tcx > , Result < Certainty , NoSolution >) ;}
mkitem!{mkimpl!{impl < 'tcx , O > Obligation < 'tcx , O > { pub fn new (tcx : TyCtxt < 'tcx > , cause : ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , predicate : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { Self :: with_depth (tcx , cause , 0 , param_env , predicate) } # [doc = " We often create nested obligations without setting the correct depth."] # [doc = ""] # [doc = " To deal with this evaluate and fulfill explicitly update the depth"] # [doc = " of nested obligations using this function."] pub fn set_depth_from_parent (& mut self , parent_depth : usize) { self . recursion_depth = cmp :: max (parent_depth + 1 , self . recursion_depth) ; } pub fn with_depth (tcx : TyCtxt < 'tcx > , cause : ObligationCause < 'tcx > , recursion_depth : usize , param_env : ty :: ParamEnv < 'tcx > , predicate : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { let predicate = predicate . upcast (tcx) ; Obligation { cause , param_env , recursion_depth , predicate } } pub fn misc (tcx : TyCtxt < 'tcx > , span : Span , body_id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , trait_ref : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { Obligation :: new (tcx , ObligationCause :: misc (span , body_id) , param_env , trait_ref) } pub fn with < P > (& self , tcx : TyCtxt < 'tcx > , value : impl Upcast < TyCtxt < 'tcx > , P > ,) -> Obligation < 'tcx , P > { Obligation :: with_depth (tcx , self . cause . clone () , self . recursion_depth , self . param_env , value) } }}}
mkitem!{mkimpl!{impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn polarity (& self) -> ty :: PredicatePolarity { self . predicate . skip_binder () . polarity } pub fn self_ty (& self) -> ty :: Binder < 'tcx , Ty < 'tcx > > { self . predicate . map_bound (| p | p . self_ty ()) } }}}