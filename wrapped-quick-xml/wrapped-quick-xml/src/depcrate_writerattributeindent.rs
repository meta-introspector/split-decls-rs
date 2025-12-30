// Generated macro for AttributeIndent (enum)
macro_rules! Depcrate_writerAttributeIndent {
() => {
// Module: crate::writer
// Provides: {"AttributeIndent"}
// Dependencies: {}
# [doc = " Track indent inside elements state"] # [doc = ""] # [doc = " ```mermaid"] # [doc = " stateDiagram-v2"] # [doc = "     [*] --> NoneAttributesWritten"] # [doc = "     NoneAttributesWritten --> Spaces : .with_attribute()"] # [doc = "     NoneAttributesWritten --> WriteConfigured : .new_line()"] # [doc = ""] # [doc = "     Spaces --> Spaces : .with_attribute()"] # [doc = "     Spaces --> WriteSpaces : .new_line()"] # [doc = ""] # [doc = "     WriteSpaces --> Spaces : .with_attribute()"] # [doc = "     WriteSpaces --> WriteSpaces : .new_line()"] # [doc = ""] # [doc = "     Configured --> Configured : .with_attribute()"] # [doc = "     Configured --> WriteConfigured : .new_line()"] # [doc = ""] # [doc = "     WriteConfigured --> Configured : .with_attribute()"] # [doc = "     WriteConfigured --> WriteConfigured : .new_line()"] # [doc = " ```"] # [derive (Debug)] enum AttributeIndent { # [doc = " Initial state. `ElementWriter` was just created and no attributes written yet"] NoneAttributesWritten , # [doc = " Write specified count of spaces to indent before writing attribute in `with_attribute()`"] WriteSpaces (usize) , # [doc = " Keep space indent that should be used if `new_line()` would be called"] Spaces (usize) , # [doc = " Write specified count of indent characters before writing attribute in `with_attribute()`"] WriteConfigured (usize) , # [doc = " Keep indent that should be used if `new_line()` would be called"] Configured (usize) , }
};
}
