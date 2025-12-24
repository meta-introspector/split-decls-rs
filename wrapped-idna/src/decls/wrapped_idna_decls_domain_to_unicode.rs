use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The [domain to Unicode](https://url.spec.whatwg.org/#concept-domain-to-unicode) algorithm;
/// version returning `String` and no ASCII deny list (i.e. _UseSTD3ASCIIRules=false_).
///
/// This function exists for backward-compatibility. Consider using [`Uts46::to_user_interface`]
/// or [`Uts46::to_unicode`].
///
/// Return the Unicode representation of a domain name,
/// normalizing characters (upper-case to lower-case and other kinds of equivalence)
/// and decoding Punycode as necessary.
///
/// If the second item of the tuple indicates an error, the first item of the tuple
/// denotes errors using the REPLACEMENT CHARACTERs in order to be able to illustrate
/// errors to the user. When the second item of the return tuple signals an error,
/// the first item of the tuple must not be used in a network protocol.
pub fn domain_to_unicode(domain: &str) -> (String, Result<(), Errors>) {
    let (cow, result) = Uts46::new()
        .to_unicode(
            domain.as_bytes(),
            uts46::AsciiDenyList::EMPTY,
            uts46::Hyphens::Allow,
        );
    (cow.into_owned(), result)
}
