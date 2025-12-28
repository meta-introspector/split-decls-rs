macro_rules! deps {
    () => {
        Error!();
        PointerKind!();
        FnCtxt!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { # [doc = " Returns the kind of unsize information of t, or None"] # [doc = " if t is unknown."] fn pointer_kind (& self , t : Ty < 'tcx > , span : Span ,) -> Result < Option < PointerKind < 'tcx > > , ErrorGuaranteed > { debug ! ("pointer_kind({:?}, {:?})" , t , span) ; let t = self . resolve_vars_if_possible (t) ; t . error_reported () ? ; if self . type_is_sized_modulo_regions (self . param_env , t) { return Ok (Some (PointerKind :: Thin)) ; } let t = self . try_structurally_resolve_type (span , t) ; Ok (match * t . kind () { ty :: Slice (_) | ty :: Str => Some (PointerKind :: Length) , ty :: Dynamic (tty , _ , ty :: Dyn) => Some (PointerKind :: VTable (tty)) , ty :: Adt (def , args) if def . is_struct () => match def . non_enum_variant () . tail_opt () { None => Some (PointerKind :: Thin) , Some (f) => { let field_ty = self . field_ty (span , f , args) ; self . pointer_kind (field_ty , span) ? } } , ty :: Tuple (fields) => match fields . last () { None => Some (PointerKind :: Thin) , Some (& f) => self . pointer_kind (f , span) ? , } , ty :: UnsafeBinder (_) => todo ! ("FIXME(unsafe_binder)") , ty :: Foreign (..) => Some (PointerKind :: Thin) , ty :: Alias (_ , pi) => Some (PointerKind :: OfAlias (pi)) , ty :: Param (p) => Some (PointerKind :: OfParam (p)) , ty :: Placeholder (..) | ty :: Bound (..) | ty :: Infer (_) => None , ty :: Bool | ty :: Char | ty :: Int (..) | ty :: Uint (..) | ty :: Float (_) | ty :: Array (..) | ty :: CoroutineWitness (..) | ty :: RawPtr (_ , _) | ty :: Ref (..) | ty :: Pat (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: Adt (..) | ty :: Never | ty :: Error (_) => { let guar = self . dcx () . span_delayed_bug (span , format ! ("`{t:?}` should be sized but is not?")) ; return Err (guar) ; } }) } }
    };
}

impl_13!()