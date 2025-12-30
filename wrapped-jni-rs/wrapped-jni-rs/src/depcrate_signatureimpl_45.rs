// Generated macro for impl_45 (impl)
macro_rules! Depcrate_signatureimpl_45 {
() => {
// Module: crate::signature
// Provides: {"impl_45"}
// Dependencies: {}
impl TypeSignature { # [doc = " Parse a signature string into a TypeSignature enum."] # [allow (clippy :: should_implement_trait)] pub fn from_str < S : AsRef < str > > (s : S) -> Result < TypeSignature > { parser (parse_sig) . parse (s . as_ref ()) . map_err (| e | Error :: ParseFailed (format ! ("Failed to parse '{}': {e}" , s . as_ref ()))) . map (| (sig , tail) | { if tail . is_empty () { Ok (sig) } else { Err (Error :: ParseFailed (format ! ("Trailing input: '{tail}' while parsing '{}'" , s . as_ref ()))) } }) ? } }
};
}
