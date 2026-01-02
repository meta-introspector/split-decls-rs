// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_attr_parsing/src/attributes/prototype.rs
// Error: expected square brackets
// Problematic line: line 17


pub(crate) struct CustomMirParser;

impl<S: Stage> SingleAttributeParser<S> for CustomMirParser {
    const PATH: &[rustc_span::Symbol] = &[sym::custom_mir];

    const ATTRIBUTE_ORDER: AttributeOrder = AttributeOrder::KeepOutermost;
