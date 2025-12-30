// Generated macro for impl_465 (impl)
macro_rules! Depcrate_harfbuzzimpl_465 {
() => {
// Module: crate::harfbuzz
// Provides: {"impl_465"}
// Dependencies: {}
impl HarfbuzzScriptData { # [doc = " Construct a new [`HarfbuzzScriptData`] using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub fn new () -> HarfbuzzScriptDataBorrowed < 'static > { HarfbuzzScriptDataBorrowed { script : CodePointMapData :: < Script > :: new () , script_names : PropertyNamesShort :: < Script > :: new () , } } # [doc = " Construct a new [`HarfbuzzScriptData`] from a data provider."] pub fn try_new_unstable < D > (provider : & D) -> Result < Self , DataError > where D : DataProvider < PropertyEnumScriptV1 > + DataProvider < PropertyNameShortScriptV1 > + ? Sized , { let script_set = CodePointMapData :: < Script > :: try_new_unstable (provider) ? ; let script_names = PropertyNamesShort :: try_new_unstable (provider) ? ; Ok (Self { script : script_set , script_names , }) } # [cfg (feature = "serde")] # [doc = icu_provider :: gen_buffer_unstable_docs ! (BUFFER , Self :: try_new_unstable)] pub fn try_new_with_buffer_provider (provider : & (impl icu_provider :: buf :: BufferProvider + ? Sized) ,) -> Result < Self , DataError > { Self :: try_new_unstable (& provider . as_deserializing ()) } fn as_borrowed (& self) -> HarfbuzzScriptDataBorrowed < '_ > { HarfbuzzScriptDataBorrowed { script : self . script . as_borrowed () , script_names : self . script_names . as_borrowed () , } } }
};
}
