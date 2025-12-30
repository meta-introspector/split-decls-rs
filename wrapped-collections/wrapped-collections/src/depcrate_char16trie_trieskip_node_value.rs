// Generated macro for skip_node_value (function)
macro_rules! Depcrate_char16trie_trieskip_node_value {
() => {
// Module: crate::char16trie::trie
// Provides: {"skip_node_value"}
// Dependencies: {}
fn skip_node_value (pos : usize , lead : u16) -> usize { if lead < MIN_TWO_UNIT_NODE_VALUE_LEAD { pos } else if lead < THREE_UNIT_NODE_VALUE_LEAD { pos + 1 } else { pos + 2 } }
};
}
