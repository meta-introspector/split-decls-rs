mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use crate :: data_structures :: HashSet ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: lang_items :: SolverTraitLangItem ;}
mkuse!{use crate :: outlives :: { Component , push_outlives_components } ;}
mkuse!{use crate :: { self as ty , Interner , Upcast as _ } ;}
mkitem!{mkstruct!{# [doc = " \"Elaboration\" is the process of identifying all the predicates that"] # [doc = " are implied by a source predicate. Currently, this basically means"] # [doc = " walking the \"supertraits\" and other similar assumptions. For example,"] # [doc = " if we know that `T: Ord`, the elaborator would deduce that `T: PartialOrd`"] # [doc = " holds as well. Similarly, if we have `trait Foo: 'static`, and we know that"] # [doc = " `T: Foo`, then we know that `T: 'static`."] pub struct Elaborator < I : Interner , O > { cx : I , stack : Vec < O > , visited : HashSet < ty :: Binder < I , ty :: PredicateKind < I > > > , mode : Filter , elaborate_sized : ElaborateSized , }}}
mkitem!{mkenum!{enum Filter { All , OnlySelf , }}}
mkitem!{mkenum!{# [derive (Eq , PartialEq)] enum ElaborateSized { Yes , No , }}}
mkitem!{mktrait!{# [doc = " Describes how to elaborate an obligation into a sub-obligation."] pub trait Elaboratable < I : Interner > { fn predicate (& self) -> I :: Predicate ; fn child (& self , clause : I :: Clause) -> Self ; fn child_with_derived_cause (& self , clause : I :: Clause , span : I :: Span , parent_trait_pred : ty :: Binder < I , ty :: TraitPredicate < I > > , index : usize ,) -> Self ; }}}
mkitem!{mkstruct!{pub struct ClauseWithSupertraitSpan < I : Interner > { pub clause : I :: Clause , pub supertrait_span : I :: Span , }}}
mkitem!{mkimpl!{impl < I : Interner > ClauseWithSupertraitSpan < I > { pub fn new (clause : I :: Clause , span : I :: Span) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span : span } } }}}
mkitem!{mkimpl!{impl < I : Interner > Elaboratable < I > for ClauseWithSupertraitSpan < I > { fn predicate (& self) -> < I as Interner > :: Predicate { self . clause . as_predicate () } fn child (& self , clause : < I as Interner > :: Clause) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span : self . supertrait_span } } fn child_with_derived_cause (& self , clause : < I as Interner > :: Clause , supertrait_span : < I as Interner > :: Span , _parent_trait_pred : crate :: Binder < I , crate :: TraitPredicate < I > > , _index : usize ,) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span } } }}}

macro_rules! elaborate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elaborate in module {}", module_path!());
    };
}

mkfn!{
    elaborate_introspect!();
    pub fn elaborate < I : Interner , O : Elaboratable < I > > (cx : I , obligations : impl IntoIterator < Item = O > ,) -> Elaborator < I , O > { let mut elaborator = Elaborator { cx , stack : Vec :: new () , visited : HashSet :: default () , mode : Filter :: All , elaborate_sized : ElaborateSized :: No , } ; elaborator . extend_deduped (obligations) ; elaborator }
}
mkitem!{mkimpl!{impl < I : Interner , O : Elaboratable < I > > Elaborator < I , O > { # [doc = " Adds `obligations` to the stack."] fn extend_deduped (& mut self , obligations : impl IntoIterator < Item = O >) { self . stack . extend (obligations . into_iter () . filter (| o | { self . visited . insert (self . cx . anonymize_bound_vars (o . predicate () . kind ())) }) ,) ; } # [doc = " Filter to only the supertraits of trait predicates, i.e. only the predicates"] # [doc = " that have `Self` as their self type, instead of all implied predicates."] pub fn filter_only_self (mut self) -> Self { self . mode = Filter :: OnlySelf ; self } # [doc = " Start elaborating `Sized` - reqd during coherence checking, normally skipped to improve"] # [doc = " compiler performance."] pub fn elaborate_sized (mut self) -> Self { self . elaborate_sized = ElaborateSized :: Yes ; self } fn elaborate (& mut self , elaboratable : & O) { let cx = self . cx ; let Some (clause) = elaboratable . predicate () . as_clause () else { return ; } ; if self . elaborate_sized == ElaborateSized :: No && let Some (did) = clause . as_trait_clause () . map (| c | c . def_id ()) && self . cx . is_trait_lang_item (did , SolverTraitLangItem :: Sized) { return ; } let bound_clause = clause . kind () ; match bound_clause . skip_binder () { ty :: ClauseKind :: Trait (data) => { if data . polarity != ty :: PredicatePolarity :: Positive { return ; } let map_to_child_clause = | (index , (clause , span)) : (usize , (I :: Clause , I :: Span)) | { elaboratable . child_with_derived_cause (clause . instantiate_supertrait (cx , bound_clause . rebind (data . trait_ref)) , span , bound_clause . rebind (data) , index ,) } ; match self . mode { Filter :: All => self . extend_deduped (cx . explicit_implied_predicates_of (data . def_id () . into ()) . iter_identity () . enumerate () . map (map_to_child_clause) ,) , Filter :: OnlySelf => self . extend_deduped (cx . explicit_super_predicates_of (data . def_id ()) . iter_identity () . enumerate () . map (map_to_child_clause) ,) , } ; } ty :: ClauseKind :: HostEffect (data) => self . extend_deduped (cx . explicit_implied_const_bounds (data . def_id () . into ()) . iter_identity () . map (| trait_ref | { elaboratable . child (trait_ref . to_host_effect_clause (cx , data . constness) . instantiate_supertrait (cx , bound_clause . rebind (data . trait_ref)) ,) } ,) ,) , ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty_max , r_min)) => { if r_min . is_bound () { return ; } let mut components = smallvec ! [] ; push_outlives_components (cx , ty_max , & mut components) ; self . extend_deduped (components . into_iter () . filter_map (| component | elaborate_component_to_clause (cx , component , r_min)) . map (| clause | elaboratable . child (bound_clause . rebind (clause) . upcast (cx))) ,) ; } ty :: ClauseKind :: RegionOutlives (..) => { } ty :: ClauseKind :: WellFormed (..) => { } ty :: ClauseKind :: Projection (..) => { } ty :: ClauseKind :: ConstEvaluatable (..) => { } ty :: ClauseKind :: ConstArgHasType (..) => { } ty :: ClauseKind :: UnstableFeature (_) => { } } } }}}

macro_rules! elaborate_component_to_clause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elaborate_component_to_clause in module {}", module_path!());
    };
}

mkfn!{
    elaborate_component_to_clause_introspect!();
    fn elaborate_component_to_clause < I : Interner > (cx : I , component : Component < I > , outlives_region : I :: Region ,) -> Option < ty :: ClauseKind < I > > { match component { Component :: Region (r) => { if r . is_bound () { None } else { Some (ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (r , outlives_region))) } } Component :: Param (p) => { let ty = Ty :: new_param (cx , p) ; Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty , outlives_region))) } Component :: Placeholder (p) => { let ty = Ty :: new_placeholder (cx , p) ; Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty , outlives_region))) } Component :: UnresolvedInferenceVariable (_) => None , Component :: Alias (alias_ty) => { Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (alias_ty . to_ty (cx) , outlives_region ,))) } Component :: EscapingAlias (_) => { None } } }
}
mkitem!{mkimpl!{impl < I : Interner , O : Elaboratable < I > > Iterator for Elaborator < I , O > { type Item = O ; fn size_hint (& self) -> (usize , Option < usize >) { (self . stack . len () , None) } fn next (& mut self) -> Option < Self :: Item > { if let Some (obligation) = self . stack . pop () { self . elaborate (& obligation) ; Some (obligation) } else { None } } }}}

macro_rules! supertrait_def_ids_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function supertrait_def_ids in module {}", module_path!());
    };
}

mkfn!{
    supertrait_def_ids_introspect!();
    # [doc = " Computes the def-ids of the transitive supertraits of `trait_def_id`. This (intentionally)"] # [doc = " does not compute the full elaborated super-predicates but just the set of def-ids. It is used"] # [doc = " to identify which traits may define a given associated type to help avoid cycle errors,"] # [doc = " and to make size estimates for vtable layout computation."] pub fn supertrait_def_ids < I : Interner > (cx : I , trait_def_id : I :: TraitId ,) -> impl Iterator < Item = I :: TraitId > { let mut set = HashSet :: default () ; let mut stack = vec ! [trait_def_id] ; set . insert (trait_def_id) ; std :: iter :: from_fn (move | | { let trait_def_id = stack . pop () ? ; for (predicate , _) in cx . explicit_super_predicates_of (trait_def_id) . iter_identity () { if let ty :: ClauseKind :: Trait (data) = predicate . kind () . skip_binder () && set . insert (data . def_id ()) { stack . push (data . def_id ()) ; } } Some (trait_def_id) }) }
}

macro_rules! supertraits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function supertraits in module {}", module_path!());
    };
}

mkfn!{
    supertraits_introspect!();
    pub fn supertraits < I : Interner > (cx : I , trait_ref : ty :: Binder < I , ty :: TraitRef < I > > ,) -> FilterToTraits < I , Elaborator < I , I :: Clause > > { elaborate (cx , [trait_ref . upcast (cx)]) . filter_only_self () . filter_to_traits () }
}
mkitem!{mkimpl!{impl < I : Interner > Elaborator < I , I :: Clause > { fn filter_to_traits (self) -> FilterToTraits < I , Self > { FilterToTraits { _cx : PhantomData , base_iterator : self } } }}}
mkitem!{mkstruct!{# [doc = " A filter around an iterator of predicates that makes it yield up"] # [doc = " just trait references."] pub struct FilterToTraits < I : Interner , It : Iterator < Item = I :: Clause > > { _cx : PhantomData < I > , base_iterator : It , }}}
mkitem!{mkimpl!{impl < I : Interner , It : Iterator < Item = I :: Clause > > Iterator for FilterToTraits < I , It > { type Item = ty :: Binder < I , ty :: TraitRef < I > > ; fn next (& mut self) -> Option < ty :: Binder < I , ty :: TraitRef < I > > > { while let Some (pred) = self . base_iterator . next () { if let Some (data) = pred . as_trait_clause () { return Some (data . map_bound (| t | t . trait_ref)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . base_iterator . size_hint () ; (0 , upper) } }}}

macro_rules! elaborate_outlives_assumptions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elaborate_outlives_assumptions in module {}", module_path!());
    };
}

mkfn!{
    elaborate_outlives_assumptions_introspect!();
    pub fn elaborate_outlives_assumptions < I : Interner > (cx : I , assumptions : impl IntoIterator < Item = ty :: OutlivesPredicate < I , I :: GenericArg > > ,) -> HashSet < ty :: OutlivesPredicate < I , I :: GenericArg > > { let mut collected = HashSet :: default () ; for ty :: OutlivesPredicate (arg1 , r2) in assumptions { collected . insert (ty :: OutlivesPredicate (arg1 , r2)) ; match arg1 . kind () { ty :: GenericArgKind :: Type (ty1) => { let mut components = smallvec ! [] ; push_outlives_components (cx , ty1 , & mut components) ; for c in components { match c { Component :: Region (r1) => { if ! r1 . is_bound () { collected . insert (ty :: OutlivesPredicate (r1 . into () , r2)) ; } } Component :: Param (p) => { let ty = Ty :: new_param (cx , p) ; collected . insert (ty :: OutlivesPredicate (ty . into () , r2)) ; } Component :: Placeholder (p) => { let ty = Ty :: new_placeholder (cx , p) ; collected . insert (ty :: OutlivesPredicate (ty . into () , r2)) ; } Component :: Alias (alias_ty) => { collected . insert (ty :: OutlivesPredicate (alias_ty . to_ty (cx) . into () , r2)) ; } Component :: UnresolvedInferenceVariable (_) | Component :: EscapingAlias (_) => { } } } } ty :: GenericArgKind :: Lifetime (_) => { } ty :: GenericArgKind :: Const (_) => { } } } collected }
}