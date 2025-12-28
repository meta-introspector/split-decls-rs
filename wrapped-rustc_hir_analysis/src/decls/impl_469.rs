macro_rules! deps {
    () => {
        AssocItemQSelf!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl AssocItemQSelf { fn to_string (& self , tcx : TyCtxt < '_ >) -> String { match * self { Self :: Trait (def_id) => tcx . def_path_str (def_id) , Self :: TyParam (def_id , _) => tcx . hir_ty_param_name (def_id) . to_string () , Self :: SelfTyAlias => kw :: SelfUpper . to_string () , } } }
    };
}

impl_469!()