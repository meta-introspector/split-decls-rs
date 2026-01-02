mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , Representability , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { representability , representability_adt_ty , params_in_repr , .. * providers } ; }
}
mkitem!{macro_rules ! rtry { ($ e : expr) => { match $ e { e @ Representability :: Infinite (_) => return e , Representability :: Representable => { } } } ; }}

macro_rules! representability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function representability in module {}", module_path!());
    };
}

mkfn!{
    representability_introspect!();
    fn representability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Representability { match tcx . def_kind (def_id) { DefKind :: Struct | DefKind :: Union | DefKind :: Enum => { for variant in tcx . adt_def (def_id) . variants () { for field in variant . fields . iter () { rtry ! (tcx . representability (field . did . expect_local ())) ; } } Representability :: Representable } DefKind :: Field => representability_ty (tcx , tcx . type_of (def_id) . instantiate_identity ()) , def_kind => bug ! ("unexpected {def_kind:?}") , } }
}

macro_rules! representability_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function representability_ty in module {}", module_path!());
    };
}

mkfn!{
    representability_ty_introspect!();
    fn representability_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Representability { match * ty . kind () { ty :: Adt (..) => tcx . representability_adt_ty (ty) , ty :: Array (ty , _) => representability_ty (tcx , ty) , ty :: Tuple (tys) => { for ty in tys { rtry ! (representability_ty (tcx , ty)) ; } Representability :: Representable } _ => Representability :: Representable , } }
}

macro_rules! representability_adt_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function representability_adt_ty in module {}", module_path!());
    };
}

mkfn!{
    representability_adt_ty_introspect!();
    fn representability_adt_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Representability { let ty :: Adt (adt , args) = ty . kind () else { bug ! ("expected adt") } ; if let Some (def_id) = adt . did () . as_local () { rtry ! (tcx . representability (def_id)) ; } let params_in_repr = tcx . params_in_repr (adt . did ()) ; for (i , arg) in args . iter () . enumerate () { if let ty :: GenericArgKind :: Type (ty) = arg . kind () { if params_in_repr . contains (i as u32) { rtry ! (representability_ty (tcx , ty)) ; } } } Representability :: Representable }
}

macro_rules! params_in_repr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function params_in_repr in module {}", module_path!());
    };
}

mkfn!{
    params_in_repr_introspect!();
    fn params_in_repr (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> DenseBitSet < u32 > { let adt_def = tcx . adt_def (def_id) ; let generics = tcx . generics_of (def_id) ; let mut params_in_repr = DenseBitSet :: new_empty (generics . own_params . len ()) ; for variant in adt_def . variants () { for field in variant . fields . iter () { params_in_repr_ty (tcx , tcx . type_of (field . did) . instantiate_identity () , & mut params_in_repr ,) ; } } params_in_repr }
}

macro_rules! params_in_repr_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function params_in_repr_ty in module {}", module_path!());
    };
}

mkfn!{
    params_in_repr_ty_introspect!();
    fn params_in_repr_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , params_in_repr : & mut DenseBitSet < u32 >) { match * ty . kind () { ty :: Adt (adt , args) => { let inner_params_in_repr = tcx . params_in_repr (adt . did ()) ; for (i , arg) in args . iter () . enumerate () { if let ty :: GenericArgKind :: Type (ty) = arg . kind () { if inner_params_in_repr . contains (i as u32) { params_in_repr_ty (tcx , ty , params_in_repr) ; } } } } ty :: Array (ty , _) => params_in_repr_ty (tcx , ty , params_in_repr) , ty :: Tuple (tys) => tys . iter () . for_each (| ty | params_in_repr_ty (tcx , ty , params_in_repr)) , ty :: Param (param) => { params_in_repr . insert (param . index) ; } _ => { } } }
}