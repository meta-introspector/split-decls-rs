mkuse!{use derive_where :: derive_where ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use crate :: data_structures :: SsoHashSet ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: visit :: { TypeSuperVisitable , TypeVisitable , TypeVisitableExt as _ , TypeVisitor } ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{mkenum!{# [derive_where (Debug ; I : Interner)] pub enum Component < I : Interner > { Region (I :: Region) , Param (I :: ParamTy) , Placeholder (I :: PlaceholderTy) , UnresolvedInferenceVariable (ty :: InferTy) , Alias (ty :: AliasTy < I >) , EscapingAlias (Vec < Component < I > >) , }}}

macro_rules! push_outlives_components_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function push_outlives_components in module {}", module_path!());
    };
}

mkfn!{
    push_outlives_components_introspect!();
    # [doc = " Push onto `out` all the things that must outlive `'a` for the condition"] # [doc = " `ty0: 'a` to hold. Note that `ty0` must be a **fully resolved type**."] pub fn push_outlives_components < I : Interner > (cx : I , ty : I :: Ty , out : & mut SmallVec < [Component < I > ; 4] > ,) { ty . visit_with (& mut OutlivesCollector { cx , out , visited : Default :: default () }) ; }
}
mkitem!{mkstruct!{struct OutlivesCollector < 'a , I : Interner > { cx : I , out : & 'a mut SmallVec < [Component < I > ; 4] > , visited : SsoHashSet < I :: Ty > , }}}
mkitem!{mkimpl!{impl < I : Interner > TypeVisitor < I > for OutlivesCollector < '_ , I > { # [cfg (not (feature = "nightly"))] type Result = () ; fn visit_ty (& mut self , ty : I :: Ty) -> Self :: Result { if ! self . visited . insert (ty) { return ; } match ty . kind () { ty :: FnDef (_ , args) => { for child in args . iter () { match child . kind () { ty :: GenericArgKind :: Lifetime (_) => { } ty :: GenericArgKind :: Type (_) | ty :: GenericArgKind :: Const (_) => { child . visit_with (self) ; } } } } ty :: Closure (_ , args) => { args . as_closure () . tupled_upvars_ty () . visit_with (self) ; } ty :: CoroutineClosure (_ , args) => { args . as_coroutine_closure () . tupled_upvars_ty () . visit_with (self) ; } ty :: Coroutine (_ , args) => { args . as_coroutine () . tupled_upvars_ty () . visit_with (self) ; args . as_coroutine () . resume_ty () . visit_with (self) ; } ty :: CoroutineWitness (..) => { } ty :: Param (p) => { self . out . push (Component :: Param (p)) ; } ty :: Placeholder (p) => { self . out . push (Component :: Placeholder (p)) ; } ty :: Alias (kind , alias_ty) => { if ! alias_ty . has_escaping_bound_vars () { self . out . push (Component :: Alias (alias_ty)) ; } else { let mut subcomponents = smallvec ! [] ; compute_alias_components_recursive (self . cx , kind , alias_ty , & mut subcomponents) ; self . out . push (Component :: EscapingAlias (subcomponents . into_iter () . collect ())) ; } } ty :: Infer (infer_ty) => { self . out . push (Component :: UnresolvedInferenceVariable (infer_ty)) ; } ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str | ty :: Never | ty :: Error (_) => { } ty :: Bound (_ , _) => { } ty :: Adt (_ , _) | ty :: Foreign (_) | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: Ref (_ , _ , _) | ty :: FnPtr (..) | ty :: UnsafeBinder (_) | ty :: Dynamic (_ , _ , _) | ty :: Tuple (_) => { ty . super_visit_with (self) ; } } } fn visit_region (& mut self , lt : I :: Region) -> Self :: Result { if ! lt . is_bound () { self . out . push (Component :: Region (lt)) ; } } }}}

macro_rules! compute_alias_components_recursive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_alias_components_recursive in module {}", module_path!());
    };
}

mkfn!{
    compute_alias_components_recursive_introspect!();
    # [doc = " Collect [Component]s for *all* the args of `alias_ty`."] # [doc = ""] # [doc = " This should not be used to get the components of `alias_ty` itself."] # [doc = " Use [push_outlives_components] instead."] pub fn compute_alias_components_recursive < I : Interner > (cx : I , kind : ty :: AliasTyKind , alias_ty : ty :: AliasTy < I > , out : & mut SmallVec < [Component < I > ; 4] > ,) { let opt_variances = cx . opt_alias_variances (kind , alias_ty . def_id) ; let mut visitor = OutlivesCollector { cx , out , visited : Default :: default () } ; for (index , child) in alias_ty . args . iter () . enumerate () { if opt_variances . and_then (| variances | variances . get (index)) == Some (ty :: Bivariant) { continue ; } child . visit_with (& mut visitor) ; } }
}