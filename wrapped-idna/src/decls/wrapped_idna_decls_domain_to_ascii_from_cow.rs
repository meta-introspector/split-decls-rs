use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;
/// version accepting and returning a `Cow`.
///
/// Most applications should be using this function or `domain_to_ascii_cow` rather
/// than the sibling functions, and most applications should pass [`AsciiDenyList::URL`] as
/// the second argument. Passing [`AsciiDenyList::URL`] as the second argument makes this function also
/// perform the [forbidden domain code point](https://url.spec.whatwg.org/#forbidden-domain-code-point)
/// check in addition to the [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii)
/// algorithm.
///
/// Return the ASCII representation a domain name,
/// normalizing characters (upper-case to lower-case and other kinds of equivalence)
/// and using Punycode as necessary.
///
/// This process may fail.
pub fn domain_to_ascii_from_cow(
    domain: Cow<'_, [u8]>,
    ascii_deny_list: AsciiDenyList,
) -> Result<Cow<'_, str>, Errors> {
    Uts46::new()
        .to_ascii_from_cow(
            domain,
            ascii_deny_list,
            uts46::Hyphens::Allow,
            uts46::DnsLength::Ignore,
        )
}
