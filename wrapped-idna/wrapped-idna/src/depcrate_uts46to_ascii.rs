// Generated macro for to_ascii (function)
macro_rules! Depcrate_uts46to_ascii {
() => {
// Module: crate::uts46
// Provides: {"to_ascii"}
// Dependencies: {}
# [doc = " http://www.unicode.org/reports/tr46/#ToASCII"] pub fn to_ascii (domain : & str , flags : Flags) -> Result < String , Errors > { let mut errors = Vec :: new () ; let mut result = String :: new () ; let mut first = true ; for label in processing (domain , flags , & mut errors) . split ('.') { if ! first { result . push ('.') ; } first = false ; if label . is_ascii () { result . push_str (label) ; } else { match punycode :: encode_str (label) { Some (x) => { result . push_str (PUNYCODE_PREFIX) ; result . push_str (& x) ; } , None => errors . push (Error :: PunycodeError) } } } if flags . verify_dns_length { let domain = if result . ends_with (".") { & result [.. result . len () - 1] } else { & * result } ; if domain . len () < 1 || domain . split ('.') . any (| label | label . len () < 1) { errors . push (Error :: TooShortForDns) } if domain . len () > 253 || domain . split ('.') . any (| label | label . len () > 63) { errors . push (Error :: TooLongForDns) } } if errors . is_empty () { Ok (result) } else { Err (Errors (errors)) } }
};
}
