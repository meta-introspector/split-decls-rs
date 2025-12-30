// Generated macro for domain_to_ascii (function)
macro_rules! Depcratedomain_to_ascii {
() => {
// Module: crate
// Provides: {"domain_to_ascii"}
// Dependencies: {}
# [doc = " The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;"] # [doc = " version returning `String` and no ASCII deny list (i.e. _UseSTD3ASCIIRules=false_)."] # [doc = ""] # [doc = " This function exists for backward-compatibility. Consider using [`domain_to_ascii_cow`]"] # [doc = " instead."] # [doc = ""] # [doc = " Return the ASCII representation a domain name,"] # [doc = " normalizing characters (upper-case to lower-case and other kinds of equivalence)"] # [doc = " and using Punycode as necessary."] # [doc = ""] # [doc = " This process may fail."] pub fn domain_to_ascii (domain : & str) -> Result < String , Errors > { domain_to_ascii_cow (domain . as_bytes () , AsciiDenyList :: EMPTY) . map (| cow | cow . into_owned ()) }
};
}
