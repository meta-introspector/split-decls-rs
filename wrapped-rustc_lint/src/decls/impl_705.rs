macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for NonUpperCaseGlobals { fn check_item (& mut self , cx : & LateContext < '_ > , it : & hir :: Item < '_ >) { let attrs = cx . tcx . hir_attrs (it . hir_id ()) ; match it . kind { hir :: ItemKind :: Static (_ , ident , ..) if ! find_attr ! (attrs , AttributeKind :: NoMangle (..)) => { NonUpperCaseGlobals :: check_upper_case (cx , "static variable" , Some (it . owner_id . def_id) , & ident ,) ; } hir :: ItemKind :: Const (ident , ..) => { NonUpperCaseGlobals :: check_upper_case (cx , "constant" , Some (it . owner_id . def_id) , & ident ,) ; } _ => { } } } fn check_trait_item (& mut self , cx : & LateContext < '_ > , ti : & hir :: TraitItem < '_ >) { if let hir :: TraitItemKind :: Const (..) = ti . kind { NonUpperCaseGlobals :: check_upper_case (cx , "associated constant" , None , & ti . ident) ; } } fn check_impl_item (& mut self , cx : & LateContext < '_ > , ii : & hir :: ImplItem < '_ >) { if let hir :: ImplItemKind :: Const (..) = ii . kind && let hir :: ImplItemImplKind :: Inherent { .. } = ii . impl_kind { NonUpperCaseGlobals :: check_upper_case (cx , "associated constant" , None , & ii . ident) ; } } fn check_pat (& mut self , cx : & LateContext < '_ > , p : & hir :: Pat < '_ >) { if let PatKind :: Expr (hir :: PatExpr { kind : PatExprKind :: Path (hir :: QPath :: Resolved (None , path)) , .. }) = p . kind { if let Res :: Def (DefKind :: Const , _) = path . res && let [segment] = path . segments { NonUpperCaseGlobals :: check_upper_case (cx , "constant in pattern" , None , & segment . ident ,) ; } } } fn check_generic_param (& mut self , cx : & LateContext < '_ > , param : & hir :: GenericParam < '_ >) { if let GenericParamKind :: Const { .. } = param . kind { NonUpperCaseGlobals :: check_upper_case (cx , "const parameter" , Some (param . def_id) , & param . name . ident () ,) ; } } }
    };
}

impl_705!()