// Generated macro for Controls (type)
macro_rules! Depcrate_controlsControls {
() => {
// Module: crate::controls
// Provides: {"Controls"}
// Dependencies: {}
# [doc = " The `Controls` type is defined in [RFC 4211 Section 6]."] # [doc = ""] # [doc = " ```text"] # [doc = "   Controls  ::= SEQUENCE SIZE(1..MAX) OF SingleAttribute"] # [doc = "                     {{RegControlSet}}"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 6]: https://www.rfc-editor.org/rfc/rfc4211#section-6"] pub type Controls = Vec < AttributeTypeAndValue > ;
};
}
