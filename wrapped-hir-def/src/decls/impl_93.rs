macro_rules! deps {
    () => {
        ConstSignature!();
        ExpressionStoreSourceMap!();
        DefDatabase!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl ConstSignature { pub fn query (db : & dyn DefDatabase , id : ConstId) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let module = loc . container . module (db) ; let attrs = db . attrs (id . into ()) ; let mut flags = ConstFlags :: empty () ; if attrs . by_key (sym :: rustc_allow_incoherent_impl) . exists () { flags |= ConstFlags :: RUSTC_ALLOW_INCOHERENT_IMPL ; } let source = loc . source (db) ; if source . value . body () . is_some () { flags . insert (ConstFlags :: HAS_BODY) ; } let (store , source_map , type_ref) = crate :: expr_store :: lower :: lower_type_ref (db , module , source . as_ref () . map (| it | it . ty ())) ; (Arc :: new (ConstSignature { store : Arc :: new (store) , type_ref , flags , name : source . value . name () . map (| it | it . as_name ()) , }) , Arc :: new (source_map) ,) } pub fn has_body (& self) -> bool { self . flags . contains (ConstFlags :: HAS_BODY) } }
    };
}

impl_93!();