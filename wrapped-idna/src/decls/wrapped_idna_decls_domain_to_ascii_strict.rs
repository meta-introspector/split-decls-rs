use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm,
/// with the `beStrict` flag set.
///
/// Note that this rejects various real-world names including:
/// * YouTube CDN nodes
/// * Some GitHub user pages
/// * Pseudo-hosts used by various TXT record-based protocols.
pub fn domain_to_ascii_strict(domain: &str) -> Result<String, Errors> {
    Uts46::new()
        .to_ascii(
            domain.as_bytes(),
            uts46::AsciiDenyList::STD3,
            uts46::Hyphens::Check,
            uts46::DnsLength::Verify,
        )
        .map(|cow| cow.into_owned())
}
