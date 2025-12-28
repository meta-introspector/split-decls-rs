macro_rules! extract_def_id_from_arg {
    () => {
        fn extract_def_id_from_arg < 'tcx > (tcx : TyCtxt < 'tcx > , generics : & 'tcx ty :: Generics , arg : ty :: GenericArg < 'tcx > ,) -> DefId { match arg . kind () { ty :: GenericArgKind :: Lifetime (re) => match re . kind () { ty :: ReEarlyParam (ebr) => generics . region_param (ebr , tcx) . def_id , ty :: ReBound (_ , ty :: BoundRegion { kind : ty :: BoundRegionKind :: Named (def_id) , .. }) | ty :: ReLateParam (ty :: LateParamRegion { scope : _ , kind : ty :: LateParamRegionKind :: Named (def_id) , }) => def_id , _ => unreachable ! () , } , ty :: GenericArgKind :: Type (ty) => { let ty :: Param (param_ty) = * ty . kind () else { bug ! () ; } ; generics . type_param (param_ty , tcx) . def_id } ty :: GenericArgKind :: Const (ct) => { let ty :: ConstKind :: Param (param_ct) = ct . kind () else { bug ! () ; } ; generics . const_param (param_ct , tcx) . def_id } } }
    };
}

extract_def_id_from_arg!();