// Generated macro for RFC822Name (struct)
macro_rules! Depcrate_typesRFC822Name {
() => {
// Module: crate::types
// Provides: {"RFC822Name"}
// Dependencies: {}
# [doc = " An `RFC822Name` represents an email address, as defined in [RFC 822 6.1]"] # [doc = " and as amended by [RFC 2821 4.1.2]. In particular, it represents the `Mailbox`"] # [doc = " rule from RFC 2821's grammar."] # [doc = ""] # [doc = " This type does not currently support the quoted local-part form; email"] # [doc = " addresses that use this form will be rejected."] # [doc = ""] # [doc = " [RFC 822 6.1]: https://datatracker.ietf.org/doc/html/rfc822#section-6.1"] # [doc = " [RFC 2821 4.1.2]: https://datatracker.ietf.org/doc/html/rfc2821#section-4.1.2"] # [derive (PartialEq)] pub struct RFC822Name < 'a > { pub mailbox : IA5String < 'a > , pub domain : DNSName < 'a > , }
};
}
