// Generated macro for impl_144 (impl)
macro_rules! Depcrate_signaturesimpl_144 {
() => {
// Module: crate::signatures
// Provides: {"impl_144"}
// Dependencies: {}
impl EnumSignature { pub fn query (db : & dyn DefDatabase , id : EnumId) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let attrs = db . attrs (id . into ()) ; let mut flags = EnumFlags :: empty () ; if attrs . by_key (sym :: rustc_has_incoherent_inherent_impls) . exists () { flags |= EnumFlags :: RUSTC_HAS_INCOHERENT_INHERENT_IMPLS ; } let repr = attrs . repr () ; let InFile { file_id , value : source } = loc . source (db) ; let (store , generic_params , source_map) = lower_generic_params (db , loc . container , id . into () , file_id , source . generic_param_list () , source . where_clause () ,) ; (Arc :: new (EnumSignature { generic_params , store , flags , repr , name : as_name_opt (source . name ()) , }) , Arc :: new (source_map) ,) } pub fn variant_body_type (& self) -> IntegerType { match self . repr { Some (ReprOptions { int : Some (builtin) , .. }) => builtin , _ => IntegerType :: Pointer (true) , } } }
};
}
