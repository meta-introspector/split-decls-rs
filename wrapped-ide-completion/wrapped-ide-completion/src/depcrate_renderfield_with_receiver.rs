// Generated macro for field_with_receiver (function)
macro_rules! Depcrate_renderfield_with_receiver {
() => {
// Module: crate::render
// Provides: {"field_with_receiver"}
// Dependencies: {}
fn field_with_receiver (receiver : Option < & str > , field_name : & str) -> SmolStr { receiver . map_or_else (| | field_name . into () , | receiver | format_smolstr ! ("{}.{field_name}" , receiver)) }
};
}
