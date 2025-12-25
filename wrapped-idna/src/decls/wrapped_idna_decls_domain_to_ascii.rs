use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
/// version returning `String` and no ASCII deny list (i.e. _UseSTD3ASCIIRules=false_).
///
/// This function exists for backward-compatibility. Consider using [`domain_to_ascii_cow`]
/// instead.
///
/// Return the ASCII representation a domain name,
/// normalizing characters (upper-case to lower-case and other kinds of equivalence)
/// and using Punycode as necessary.
///
/// This process may fail.
pub fn domain_to_ascii(domain: &str) -> Result<String, Errors> {
    domain_to_ascii_cow(domain.as_bytes(), AsciiDenyList::EMPTY).map(|cow| cow.into_owned())
}
