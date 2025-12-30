// Generated macro for impl_150 (impl)
macro_rules! Depcrate_signaturesimpl_150 {
() => {
// Module: crate::signatures
// Provides: {"impl_150"}
// Dependencies: {}
impl StaticSignature { pub fn query (db : & dyn DefDatabase , id : StaticId) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let module = loc . container . module (db) ; let attrs = db . attrs (id . into ()) ; let mut flags = StaticFlags :: empty () ; if attrs . by_key (sym :: rustc_allow_incoherent_impl) . exists () { flags |= StaticFlags :: RUSTC_ALLOW_INCOHERENT_IMPL ; } if matches ! (loc . container , ItemContainerId :: ExternBlockId (_)) { flags . insert (StaticFlags :: EXTERN) ; } let source = loc . source (db) ; if source . value . body () . is_some () { flags . insert (StaticFlags :: HAS_BODY) ; } if source . value . mut_token () . is_some () { flags . insert (StaticFlags :: MUTABLE) ; } if source . value . unsafe_token () . is_some () { flags . insert (StaticFlags :: UNSAFE) ; } if source . value . safe_token () . is_some () { flags . insert (StaticFlags :: EXPLICIT_SAFE) ; } let (store , source_map , type_ref) = crate :: expr_store :: lower :: lower_type_ref (db , module , source . as_ref () . map (| it | it . ty ())) ; (Arc :: new (StaticSignature { store : Arc :: new (store) , type_ref , flags , name : as_name_opt (source . value . name ()) , }) , Arc :: new (source_map) ,) } }
};
}
