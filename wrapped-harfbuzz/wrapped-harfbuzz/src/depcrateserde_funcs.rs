// Generated macro for serde_funcs (function)
macro_rules! Depcrateserde_funcs {
() => {
// Module: crate
// Provides: {"serde_funcs"}
// Dependencies: {}
pub fn serde_funcs < P : icu_provider :: buf :: BufferProvider > (provider : & P ,) -> Result < UnicodeFuncs , icu_provider :: DataError > { let provider = icu_provider :: buf :: AsDeserializingBufferProvider :: as_deserializing (provider) ; let mut builder = UnicodeFuncsBuilder :: new_with_empty_parent () . unwrap () ; builder . set_general_category_func (Box :: new (CodePointMapData :: < GeneralCategory > :: try_new_unstable (& provider) ? ,)) ; builder . set_combining_class_func (Box :: new (CanonicalCombiningClassMap :: try_new_unstable (& provider ,) ?)) ; builder . set_mirroring_func (Box :: new (CodePointMapData :: < BidiMirroringGlyph > :: try_new_unstable (& provider) ? ,)) ; builder . set_script_func (Box :: new (HarfbuzzScriptData :: try_new_unstable (& provider) ?)) ; builder . set_compose_func (Box :: new (CanonicalComposition :: try_new_unstable (& provider) ?)) ; builder . set_decompose_func (Box :: new (CanonicalDecomposition :: try_new_unstable (& provider ,) ?)) ; Ok (builder . build ()) }
};
}
