// Generated macro for impl_375 (impl)
macro_rules! Depcrate_options_viewimpl_375 {
() => {
// Module: crate::options::view
// Provides: {"impl_375"}
// Dependencies: {}
impl SizeFormat { # [doc = " Determine which file size to use in the file size column based on"] # [doc = " the user’s options."] # [doc = ""] # [doc = " The default mode is to use the decimal prefixes, as they are the"] # [doc = " most commonly-understood, and don’t involve trying to parse large"] # [doc = " strings of digits in your head. Changing the format to anything else"] # [doc = " involves the `--binary` or `--bytes` flags, and these conflict with"] # [doc = " each other."] fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let flag = matches . has_where (| f | f . matches (& flags :: BINARY) || f . matches (& flags :: BYTES)) ? ; Ok (match flag { Some (f) if f . matches (& flags :: BINARY) => Self :: BinaryBytes , Some (f) if f . matches (& flags :: BYTES) => Self :: JustBytes , _ => Self :: DecimalBytes , }) } }
};
}
