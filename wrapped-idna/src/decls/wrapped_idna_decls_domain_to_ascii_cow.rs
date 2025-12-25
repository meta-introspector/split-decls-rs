use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
/// version returning a `Cow`.
///
/// Most applications should be using this function or `domain_to_ascii_from_cow` rather
/// than the sibling functions, and most applications should pass [`AsciiDenyList::URL`] as
/// the second argument. Passing [`AsciiDenyList::URL`] as the second argument makes this function also
/// perform the [forbidden domain code point](https://url.spec.whatwg.org/#forbidden-domain-code-point)
/// check in addition to the [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii)
/// algorithm.
///
/// Returns the ASCII representation a domain name,
/// normalizing characters (upper-case to lower-case and other kinds of equivalence)
/// and using Punycode as necessary.
///
/// This process may fail.
///
/// If you have a `&str` instead of `&[u8]`, just call `.as_bytes()` on it before
/// passing it to this function. It's still preferable to use this function over
/// the sibling functions that take `&str`.
pub fn domain_to_ascii_cow(
    domain: &[u8],
    ascii_deny_list: AsciiDenyList,
) -> Result<Cow<'_, str>, Errors> {
    Uts46::new().to_ascii(
        domain,
        ascii_deny_list,
        uts46::Hyphens::Allow,
        uts46::DnsLength::Ignore,
    )
}
