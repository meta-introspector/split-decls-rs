// Generated macro for impl_905 (impl)
macro_rules! Depcrate_transliterate_compile_parseimpl_905 {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"impl_905"}
// Dependencies: {}
impl BasicId { pub (crate) fn reverse (self) -> Self { let source = self . source . to_lowercase () ; let target = self . target . to_lowercase () ; let (new_source , new_target) = match (source . as_str () , target . as_str ()) { ("any" , "lower") => (self . source , "Upper" . to_string ()) , ("any" , "upper") => (self . source , "Lower" . to_string ()) , ("any" , "nfc") => (self . source , "NFD" . to_string ()) , ("any" , "nfd") => (self . source , "NFC" . to_string ()) , ("any" , "nfkc") => (self . source , "NFKD" . to_string ()) , ("any" , "nfkd") => (self . source , "NFKC" . to_string ()) , ("any" , "remove" | "null") => (self . source , self . target) , _ => (self . target , self . source) , } ; Self { source : new_source , target : new_target , variant : self . variant , } } }
};
}
