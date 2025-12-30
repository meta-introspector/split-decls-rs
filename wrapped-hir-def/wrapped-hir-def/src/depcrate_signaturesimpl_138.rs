// Generated macro for impl_138 (impl)
macro_rules! Depcrate_signaturesimpl_138 {
() => {
// Module: crate::signatures
// Provides: {"impl_138"}
// Dependencies: {}
impl StructSignature { pub fn query (db : & dyn DefDatabase , id : StructId) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let loc = id . lookup (db) ; let InFile { file_id , value : source } = loc . source (db) ; let attrs = db . attrs (id . into ()) ; let mut flags = StructFlags :: empty () ; if attrs . by_key (sym :: rustc_has_incoherent_inherent_impls) . exists () { flags |= StructFlags :: RUSTC_HAS_INCOHERENT_INHERENT_IMPLS ; } if attrs . by_key (sym :: fundamental) . exists () { flags |= StructFlags :: FUNDAMENTAL ; } if let Some (lang) = attrs . lang_item () { match lang { LangItem :: PhantomData => flags |= StructFlags :: IS_PHANTOM_DATA , LangItem :: OwnedBox => flags |= StructFlags :: IS_BOX , LangItem :: ManuallyDrop => flags |= StructFlags :: IS_MANUALLY_DROP , LangItem :: UnsafeCell => flags |= StructFlags :: IS_UNSAFE_CELL , LangItem :: UnsafePinned => flags |= StructFlags :: IS_UNSAFE_PINNED , _ => () , } } let repr = attrs . repr () ; let shape = adt_shape (source . kind ()) ; let (store , generic_params , source_map) = lower_generic_params (db , loc . container , id . into () , file_id , source . generic_param_list () , source . where_clause () ,) ; (Arc :: new (StructSignature { generic_params , store , flags , shape , name : as_name_opt (source . name ()) , repr , }) , Arc :: new (source_map) ,) } }
};
}
