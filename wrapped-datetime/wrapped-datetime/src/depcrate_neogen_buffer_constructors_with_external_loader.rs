// Generated macro for gen_buffer_constructors_with_external_loader (macro)
macro_rules! Depcrate_neogen_buffer_constructors_with_external_loader {
() => {
// Module: crate::neo
// Provides: {"gen_buffer_constructors_with_external_loader"}
// Dependencies: {}
# [doc = " Helper macro for generating any/buffer constructors in this file."] macro_rules ! gen_buffer_constructors_with_external_loader { (@ compiletime_fset , $ fset : ident , $ compiled_fn : ident , $ buffer_fn : ident , $ internal_fn : ident) => { # [doc = icu_provider :: gen_buffer_unstable_docs ! (BUFFER , Self ::$ compiled_fn)] # [cfg (feature = "serde")] pub fn $ buffer_fn < P > (provider : & P , prefs : DateTimeFormatterPreferences , field_set_with_options : $ fset ,) -> Result < Self , DateTimeFormatterLoadError > where P : BufferProvider + ? Sized , { use crate :: provider :: compat :: CompatProvider ; let deser_provider = provider . as_deserializing () ; let compat_provider = CompatProvider (& deser_provider , & provider) ; Self ::$ internal_fn (& compat_provider , & ExternalLoaderBuffer (provider) , prefs , field_set_with_options . get_field () ,) } } ; }
};
}
