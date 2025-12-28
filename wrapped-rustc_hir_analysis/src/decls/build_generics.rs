macro_rules! deps {
    () => {
        InheritanceKind!();
    };
}

macro_rules! build_generics {
    () => {
        deps!();
        fn build_generics < 'tcx > (tcx : TyCtxt < 'tcx > , sig_id : DefId , parent : Option < DefId > , inh_kind : InheritanceKind ,) -> ty :: Generics { let mut own_params = vec ! [] ; let sig_generics = tcx . generics_of (sig_id) ; if let InheritanceKind :: WithParent (has_self) = inh_kind && let Some (parent_def_id) = sig_generics . parent { let sig_parent_generics = tcx . generics_of (parent_def_id) ; own_params . append (& mut sig_parent_generics . own_params . clone ()) ; if ! has_self { own_params . remove (0) ; } } own_params . append (& mut sig_generics . own_params . clone ()) ; own_params . sort_by_key (| key | key . kind . is_ty_or_const ()) ; let param_def_id_to_index = own_params . iter () . map (| param | (param . def_id , param . index)) . collect () ; let (parent_count , has_self) = if let Some (def_id) = parent { let parent_generics = tcx . generics_of (def_id) ; let parent_kind = tcx . def_kind (def_id) ; (parent_generics . count () , parent_kind == DefKind :: Trait) } else { (0 , false) } ; for (idx , param) in own_params . iter_mut () . enumerate () { param . index = (idx + parent_count) as u32 ; if let ty :: GenericParamDefKind :: Type { has_default , .. } | ty :: GenericParamDefKind :: Const { has_default , .. } = & mut param . kind { * has_default = false ; } } ty :: Generics { parent , parent_count , own_params , param_def_id_to_index , has_self , has_late_bound_regions : sig_generics . has_late_bound_regions , } }
    };
}

build_generics!();