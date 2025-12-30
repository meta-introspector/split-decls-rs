// Generated macro for ContextFlags (type)
macro_rules! Depcrate_negotiationContextFlags {
() => {
// Module: crate::negotiation
// Provides: {"ContextFlags"}
// Dependencies: {}
# [doc = " `ContextFlags` as defined in [RFC 4178 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " ContextFlags ::= BIT STRING {"] # [doc = "     delegFlag       (0),"] # [doc = "     mutualFlag      (1),"] # [doc = "     replayFlag      (2),"] # [doc = "     sequenceFlag    (3),"] # [doc = "     anonFlag        (4),"] # [doc = "     confFlag        (5),"] # [doc = "     integFlag       (6)"] # [doc = " } (SIZE (32))"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4178 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc4178#section-4.2.1"] pub type ContextFlags = BitString ;
};
}
