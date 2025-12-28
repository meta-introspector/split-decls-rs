macro_rules! deps {
    () => {
        ItemContainerId!();
        ExpressionStoreSourceMap!();
        DefDatabase!();
        TypeAliasSignature!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl TypeAliasSignature { pub fn query (db : & dyn DefDatabase , id : TypeAliasId ,) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let mut flags = TypeAliasFlags :: empty () ; let attrs = db . attrs (id . into ()) ; if attrs . by_key (sym :: rustc_has_incoherent_inherent_impls) . exists () { flags . insert (TypeAliasFlags :: RUSTC_HAS_INCOHERENT_INHERENT_IMPL) ; } if attrs . by_key (sym :: rustc_allow_incoherent_impl) . exists () { flags . insert (TypeAliasFlags :: RUSTC_ALLOW_INCOHERENT_IMPL) ; } if matches ! (loc . container , ItemContainerId :: ExternBlockId (_)) { flags . insert (TypeAliasFlags :: IS_EXTERN) ; } let source = loc . source (db) ; let name = as_name_opt (source . value . name ()) ; let (store , source_map , generic_params , bounds , ty) = lower_type_alias (db , loc . container . module (db) , source , id) ; (Arc :: new (TypeAliasSignature { store : Arc :: new (store) , generic_params , flags , bounds , name , ty , }) , Arc :: new (source_map) ,) } }
    };
}

impl_108!();