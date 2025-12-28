macro_rules! deps {
    () => {
        DefDatabase!();
        Scope!();
        DefMap!();
        LocalModuleId!();
        LocalDefMap!();
        BuiltinShadowMode!();
        Resolver!();
        PerNs!();
        Item!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < 'db > Resolver < 'db > { fn scopes (& self) -> impl Iterator < Item = & Scope < 'db > > { self . scopes . iter () . rev () } fn resolve_module_path (& self , db : & dyn DefDatabase , path : & ModPath , shadow : BuiltinShadowMode ,) -> PerNs { let (item_map , item_local_map , module) = self . item_scope_ () ; let (module_res , segment_index) = item_map . resolve_path (item_local_map , db , module , path , shadow , None) ; if segment_index . is_some () { return PerNs :: none () ; } module_res } # [doc = " The innermost block scope that contains items or the module scope that contains this resolver."] fn item_scope_ (& self) -> (& DefMap , & LocalDefMap , LocalModuleId) { self . scopes () . find_map (| scope | match scope { Scope :: BlockScope (m) => Some ((m . def_map , m . local_def_map , m . module_id)) , _ => None , }) . unwrap_or ((self . module_scope . def_map , self . module_scope . local_def_map , self . module_scope . module_id ,)) } }
    };
}

impl_315!()