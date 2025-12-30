// Generated macro for parse_accumulate (function)
macro_rules! Depcrate_attrparse_accumulate {
() => {
// Module: crate::attr
// Provides: {"parse_accumulate"}
// Dependencies: {}
fn parse_accumulate (ctx : Ctx , attrs : & [Attribute]) -> ParseAcc { let mut state = ParseAcc :: default () ; for attr in attrs { if is_proptest_attr (& attr) { state = extract_modifiers (ctx , & attr) . into_iter () . fold (state , | state , meta | dispatch_attribute (ctx , state , meta)) } } state }
};
}
