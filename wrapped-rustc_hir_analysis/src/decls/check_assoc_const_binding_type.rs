macro_rules! deps {
    () => {
        ParamInTyOfAssocConstBinding!();
        HirTyLowerer!();
        TyOfAssocConstBindingNote!();
        EscapingBoundVarInTyOfAssocConstBinding!();
        GenericParamAndBoundVarCollector!();
    };
}

macro_rules! check_assoc_const_binding_type {
    () => {
        deps!();
        # [doc = " Detect and reject early-bound & escaping late-bound generic params in the type of assoc const bindings."] # [doc = ""] # [doc = " FIXME(const_generics): This is a temporary and semi-artificial restriction until the"] # [doc = " arrival of *generic const generics*[^1]."] # [doc = ""] # [doc = " It might actually be possible that we can already support early-bound generic params"] # [doc = " in such types if we just lifted some more checks in other places, too, for example"] # [doc = " inside `HirTyLowerer::lower_anon_const`. However, even if that were the case, we should"] # [doc = " probably gate this behind another feature flag."] # [doc = ""] # [doc = " [^1]: <https://github.com/rust-lang/project-const-generics/issues/28>."] fn check_assoc_const_binding_type < 'tcx > (cx : & dyn HirTyLowerer < 'tcx > , assoc_const : Ident , ty : ty :: Binder < 'tcx , Ty < 'tcx > > , hir_id : hir :: HirId ,) -> Ty < 'tcx > { let ty = ty . skip_binder () ; if ! ty . has_param () && ! ty . has_escaping_bound_vars () { return ty ; } let mut collector = GenericParamAndBoundVarCollector { cx , params : Default :: default () , vars : Default :: default () , depth : ty :: INNERMOST , } ; let mut guar = ty . visit_with (& mut collector) . break_value () ; let tcx = cx . tcx () ; let ty_note = ty . make_suggestable (tcx , false , None) . map (| ty | crate :: errors :: TyOfAssocConstBindingNote { assoc_const , ty }) ; let enclosing_item_owner_id = tcx . hir_parent_owner_iter (hir_id) . find_map (| (owner_id , parent) | parent . generics () . map (| _ | owner_id)) . unwrap () ; let generics = tcx . generics_of (enclosing_item_owner_id) ; for index in collector . params { let param = generics . param_at (index as _ , tcx) ; let is_self_param = param . name == kw :: SelfUpper ; guar . get_or_insert (cx . dcx () . emit_err (crate :: errors :: ParamInTyOfAssocConstBinding { span : assoc_const . span , assoc_const , param_name : param . name , param_def_kind : tcx . def_descr (param . def_id) , param_category : if is_self_param { "self" } else if param . kind . is_synthetic () { "synthetic" } else { "normal" } , param_defined_here_label : (! is_self_param) . then (| | tcx . def_ident_span (param . def_id) . unwrap ()) , ty_note , })) ; } for var_def_id in collector . vars { guar . get_or_insert (cx . dcx () . emit_err (crate :: errors :: EscapingBoundVarInTyOfAssocConstBinding { span : assoc_const . span , assoc_const , var_name : cx . tcx () . item_name (var_def_id) , var_def_kind : tcx . def_descr (var_def_id) , var_defined_here_label : tcx . def_ident_span (var_def_id) . unwrap () , ty_note , } ,)) ; } let guar = guar . unwrap_or_else (| | bug ! ("failed to find gen params or bound vars in ty")) ; Ty :: new_error (tcx , guar) }
    };
}

check_assoc_const_binding_type!()