// Generated macro for impl_402 (impl)
macro_rules! Depcrate_scriptimpl_402 {
() => {
// Module: crate::script
// Provides: {"impl_402"}
// Dependencies: {}
impl ScriptWithExtensions { # [doc = " Creates a new instance of `ScriptWithExtensionsBorrowed` using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub fn new () -> ScriptWithExtensionsBorrowed < 'static > { ScriptWithExtensionsBorrowed :: new () } icu_provider :: gen_buffer_data_constructors ! (() -> result : Result < ScriptWithExtensions , DataError >, functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable (provider : & (impl DataProvider < PropertyScriptWithExtensionsV1 > + ? Sized) ,) -> Result < Self , DataError > { Ok (ScriptWithExtensions :: from_data (provider . load (Default :: default ()) ? . payload ,)) } # [doc = " Construct a borrowed version of this type that can be queried."] # [doc = ""] # [doc = " This avoids a potential small underlying cost per API call (ex: `contains()`) by consolidating it"] # [doc = " up front."] # [inline] pub fn as_borrowed (& self) -> ScriptWithExtensionsBorrowed < '_ > { ScriptWithExtensionsBorrowed { data : self . data . get () , } } # [doc = " Construct a new one from loaded data"] # [doc = ""] # [doc = " Typically it is preferable to use getters like [`load_script_with_extensions_unstable()`] instead"] pub (crate) fn from_data (data : DataPayload < PropertyScriptWithExtensionsV1 >) -> Self { Self { data } } }
};
}
