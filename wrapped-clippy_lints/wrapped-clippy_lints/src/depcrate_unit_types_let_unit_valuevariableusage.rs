// Generated macro for VariableUsage (enum)
macro_rules! Depcrate_unit_types_let_unit_valueVariableUsage {
() => {
// Module: crate::unit_types::let_unit_value
// Provides: {"VariableUsage"}
// Dependencies: {}
# [doc = " How the unit variable is used"] enum VariableUsage { Normal (Span) , # [doc = " Captured in a `format!`:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let unit = ();"] # [doc = " eprintln!(\"{unit}\");"] # [doc = " ```"] FormatCapture , # [doc = " In a field shorthand init:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " struct Foo {"] # [doc = "    unit: (),"] # [doc = " }"] # [doc = " let unit = ();"] # [doc = " Foo { unit };"] # [doc = " ```"] FieldShorthand (Span) , }
};
}
