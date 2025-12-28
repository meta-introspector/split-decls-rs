macro_rules! deps {
    () => {
        DefDatabase!();
        ImplSignature!();
        ExpressionStoreSourceMap!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl ImplSignature { pub fn query (db : & dyn DefDatabase , id : ImplId) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let mut flags = ImplFlags :: empty () ; let src = loc . source (db) ; if src . value . unsafe_token () . is_some () { flags . insert (ImplFlags :: UNSAFE) ; } if src . value . excl_token () . is_some () { flags . insert (ImplFlags :: NEGATIVE) ; } if src . value . default_token () . is_some () { flags . insert (ImplFlags :: DEFAULT) ; } let (store , source_map , self_ty , target_trait , generic_params) = crate :: expr_store :: lower :: lower_impl (db , loc . container , src , id) ; (Arc :: new (ImplSignature { store : Arc :: new (store) , generic_params , self_ty , target_trait , flags , }) , Arc :: new (source_map) ,) } # [inline] pub fn is_negative (& self) -> bool { self . flags . contains (ImplFlags :: NEGATIVE) } # [inline] pub fn is_default (& self) -> bool { self . flags . contains (ImplFlags :: DEFAULT) } }
    };
}

impl_99!()