macro_rules! deps {
    () => {
        UnusedGenericParameterHelp!();
        IsProbablyCyclical!();
        CollectUsageSpans!();
        RecursiveGenericParameter!();
        UnusedGenericParameter!();
    };
}

macro_rules! report_bivariance {
    () => {
        deps!();
        fn report_bivariance < 'tcx > (tcx : TyCtxt < 'tcx > , param : & 'tcx hir :: GenericParam < 'tcx > , has_explicit_bounds : bool , item : & 'tcx hir :: Item < 'tcx > ,) -> ErrorGuaranteed { let param_name = param . name . ident () ; let help = match item . kind { ItemKind :: Enum (..) | ItemKind :: Struct (..) | ItemKind :: Union (..) => { if let Some (def_id) = tcx . lang_items () . phantom_data () { errors :: UnusedGenericParameterHelp :: Adt { param_name , phantom_data : tcx . def_path_str (def_id) , } } else { errors :: UnusedGenericParameterHelp :: AdtNoPhantomData { param_name } } } ItemKind :: TyAlias (..) => errors :: UnusedGenericParameterHelp :: TyAlias { param_name } , item_kind => bug ! ("report_bivariance: unexpected item kind: {item_kind:?}") , } ; let mut usage_spans = vec ! [] ; intravisit :: walk_item (& mut CollectUsageSpans { spans : & mut usage_spans , param_def_id : param . def_id . to_def_id () } , item ,) ; if ! usage_spans . is_empty () { let item_def_id = item . owner_id . to_def_id () ; let is_probably_cyclical = IsProbablyCyclical { tcx , item_def_id , seen : Default :: default () } . visit_def (item_def_id) . is_break () ; if is_probably_cyclical { return tcx . dcx () . emit_err (errors :: RecursiveGenericParameter { spans : usage_spans , param_span : param . span , param_name , param_def_kind : tcx . def_descr (param . def_id . to_def_id ()) , help , note : () , }) ; } } let const_param_help = matches ! (param . kind , hir :: GenericParamKind :: Type { .. } if ! has_explicit_bounds) ; let mut diag = tcx . dcx () . create_err (errors :: UnusedGenericParameter { span : param . span , param_name , param_def_kind : tcx . def_descr (param . def_id . to_def_id ()) , usage_spans , help , const_param_help , }) ; diag . code (E0392) ; diag . emit () }
    };
}

report_bivariance!();