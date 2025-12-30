// Generated macro for impl_141 (impl)
macro_rules! Depcrate_signaturesimpl_141 {
() => {
// Module: crate::signatures
// Provides: {"impl_141"}
// Dependencies: {}
impl UnionSignature { pub fn query (db : & dyn DefDatabase , id : UnionId) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let attrs = db . attrs (id . into ()) ; let mut flags = StructFlags :: empty () ; if attrs . by_key (sym :: rustc_has_incoherent_inherent_impls) . exists () { flags |= StructFlags :: RUSTC_HAS_INCOHERENT_INHERENT_IMPLS ; } if attrs . by_key (sym :: fundamental) . exists () { flags |= StructFlags :: FUNDAMENTAL ; } let repr = attrs . repr () ; let InFile { file_id , value : source } = loc . source (db) ; let (store , generic_params , source_map) = lower_generic_params (db , loc . container , id . into () , file_id , source . generic_param_list () , source . where_clause () ,) ; (Arc :: new (UnionSignature { generic_params , store , flags , repr , name : as_name_opt (source . name ()) , }) , Arc :: new (source_map) ,) } }
};
}
