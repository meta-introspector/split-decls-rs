// Generated macro for NodeOrText (enum)
macro_rules! Depcrate_tree_builder_interfaceNodeOrText {
() => {
// Module: crate::tree_builder::interface
// Provides: {"NodeOrText"}
// Dependencies: {}
# [doc = " Something which can be inserted into the DOM."] # [doc = ""] # [doc = " Adjacent sibling text nodes are merged into a single node, so"] # [doc = " the sink may not want to allocate a `Handle` for each."] pub enum NodeOrText < Handle > { AppendNode (Handle) , AppendText (StrTendril) , }
};
}
