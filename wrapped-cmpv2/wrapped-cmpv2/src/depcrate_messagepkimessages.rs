// Generated macro for PkiMessages (type)
macro_rules! Depcrate_messagePkiMessages {
() => {
// Module: crate::message
// Provides: {"PkiMessages"}
// Dependencies: {}
# [doc = " The `PkiMessages` type is defined in [RFC 4210 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " PKIMessages ::= SEQUENCE SIZE (1..MAX) OF PKIMessage"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.1]: https://datatracker.ietf.org/doc/html/rfc4210#section-5.1"] pub type PkiMessages < 'a > = Vec < PkiMessage < 'a > > ;
};
}
