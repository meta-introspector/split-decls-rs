// Generated macro for impl_70 (impl)
macro_rules! Depcrate_deserializerimpl_70 {
() => {
// Module: crate::deserializer
// Provides: {"impl_70"}
// Dependencies: {}
impl DeserializeErrorKind { # [allow (deprecated)] fn description (& self) -> & str { use self :: DeserializeErrorKind :: * ; match * self { Message (_) => "deserialization error" , Unsupported (_) => "unsupported deserializer method" , UnexpectedEndOfRow => "expected field, but got end of row" , InvalidUtf8 (ref err) => err . description () , ParseBool (ref err) => err . description () , ParseInt (ref err) => err . description () , ParseFloat (ref err) => err . description () , } } }
};
}
