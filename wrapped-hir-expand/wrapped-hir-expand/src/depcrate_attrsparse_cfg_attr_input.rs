// Generated macro for parse_cfg_attr_input (function)
macro_rules! Depcrate_attrsparse_cfg_attr_input {
() => {
// Module: crate::attrs
// Provides: {"parse_cfg_attr_input"}
// Dependencies: {}
fn parse_cfg_attr_input (subtree : & TopSubtree ,) -> Option < (tt :: TokenTreesView < '_ > , impl Iterator < Item = tt :: TokenTreesView < '_ > >) > { let mut parts = subtree . token_trees () . split (| tt | matches ! (tt , tt :: TtElement :: Leaf (tt :: Leaf :: Punct (Punct { char : ',' , .. })))) ; let cfg = parts . next () ? ; Some ((cfg , parts . filter (| it | ! it . is_empty ()))) }
};
}
