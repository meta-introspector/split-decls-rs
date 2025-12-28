macro_rules! deps {
    () => {
        OnlyCurrentTraitsAdt!();
        OnlyCurrentTraitsPointerSugg!();
        OnlyCurrentTraitsName!();
        OnlyCurrentTraitsForeign!();
        OnlyCurrentTraitsPointer!();
        TyParamSome!();
        OnlyCurrentTraits!();
        TyParamFirstLocal!();
        OnlyCurrentTraitsOpaque!();
        OnlyCurrentTraitsTy!();
    };
}

macro_rules! emit_orphan_check_error {
    () => {
        deps!();
        fn emit_orphan_check_error < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > , impl_def_id : LocalDefId , err : traits :: OrphanCheckErr < TyCtxt < 'tcx > , FxIndexSet < DefId > > ,) -> ErrorGuaranteed { match err { traits :: OrphanCheckErr :: NonLocalInputType (tys) => { let item = tcx . hir_expect_item (impl_def_id) ; let impl_ = item . expect_impl () ; let of_trait = impl_ . of_trait . unwrap () ; let span = tcx . def_span (impl_def_id) ; let mut diag = tcx . dcx () . create_err (match trait_ref . self_ty () . kind () { ty :: Adt (..) => errors :: OnlyCurrentTraits :: Outside { span , note : () } , _ if trait_ref . self_ty () . is_primitive () => { errors :: OnlyCurrentTraits :: Primitive { span , note : () } } _ => errors :: OnlyCurrentTraits :: Arbitrary { span , note : () } , }) ; for & (mut ty , is_target_ty) in & tys { let span = if matches ! (is_target_ty , IsFirstInputType :: Yes) { impl_ . self_ty . span } else { of_trait . trait_ref . path . span } ; ty = tcx . erase_and_anonymize_regions (ty) ; let is_foreign = ! trait_ref . def_id . is_local () && matches ! (is_target_ty , IsFirstInputType :: No) ; match * ty . kind () { ty :: Slice (_) => { if is_foreign { diag . subdiagnostic (errors :: OnlyCurrentTraitsForeign { span }) ; } else { diag . subdiagnostic (errors :: OnlyCurrentTraitsName { span , name : "slices" , }) ; } } ty :: Array (..) => { if is_foreign { diag . subdiagnostic (errors :: OnlyCurrentTraitsForeign { span }) ; } else { diag . subdiagnostic (errors :: OnlyCurrentTraitsName { span , name : "arrays" , }) ; } } ty :: Tuple (..) => { if is_foreign { diag . subdiagnostic (errors :: OnlyCurrentTraitsForeign { span }) ; } else { diag . subdiagnostic (errors :: OnlyCurrentTraitsName { span , name : "tuples" , }) ; } } ty :: Alias (ty :: Opaque , ..) => { diag . subdiagnostic (errors :: OnlyCurrentTraitsOpaque { span }) ; } ty :: RawPtr (ptr_ty , mutbl) => { if ! trait_ref . self_ty () . has_param () { diag . subdiagnostic (errors :: OnlyCurrentTraitsPointerSugg { wrapper_span : impl_ . self_ty . span , struct_span : item . span . shrink_to_lo () , mut_key : mutbl . prefix_str () , ptr_ty , }) ; } diag . subdiagnostic (errors :: OnlyCurrentTraitsPointer { span , pointer : ty }) ; } ty :: Adt (adt_def , _) => { diag . subdiagnostic (errors :: OnlyCurrentTraitsAdt { span , name : tcx . def_path_str (adt_def . did ()) , }) ; } _ => { diag . subdiagnostic (errors :: OnlyCurrentTraitsTy { span , ty }) ; } } } diag . emit () } traits :: OrphanCheckErr :: UncoveredTyParams (UncoveredTyParams { uncovered , local_ty }) => { let mut reported = None ; for param_def_id in uncovered { let name = tcx . item_ident (param_def_id) ; let span = name . span ; reported . get_or_insert (match local_ty { Some (local_type) => tcx . dcx () . emit_err (errors :: TyParamFirstLocal { span , note : () , param : name , local_type , }) , None => tcx . dcx () . emit_err (errors :: TyParamSome { span , note : () , param : name }) , }) ; } reported . unwrap () } } }
    };
}

emit_orphan_check_error!()