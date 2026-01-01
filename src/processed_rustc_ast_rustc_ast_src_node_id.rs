// SRC: ../rust/compiler/rustc_ast/src/node_id.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=NodeId | COMPLEXITY=7 | LINES=19 */
use std::fmt;

use crate::rustc_complete::LocalExpnId;

crate::rustc_index::newtype_index! {
    /// Identifies an AST node.
    ///
    /// This identifies top-level definitions, expressions, and everything in between.
    /// This is later turned into [`DefId`] and `HirId` for the HIR.
    ///
    /// [`DefId`]: crate::rustc_span::def_id::DefId
    #[encodable]
    #[orderable]
    #[debug_format = "NodeId({})"]
    pub struct NodeId {
        /// The [`NodeId`] used to represent the root of the crate.
        const CRATE_NODE_ID = 0;
    }
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=placeholder_from_expn_id | COMPLEXITY=5 | LINES=17 */

crate::rustc_data_structures::define_id_collections!(NodeMap, NodeSet, NodeMapEntry, NodeId);

/// When parsing and at the beginning of doing expansions, we initially give all AST nodes
/// this dummy AST [`NodeId`]. Then, during a later phase of expansion, we renumber them
/// to have small, positive IDs.
pub const DUMMY_NODE_ID: NodeId = NodeId::MAX;

impl NodeId {
    pub fn placeholder_from_expn_id(expn_id: LocalExpnId) -> Self {
        NodeId::from_u32(expn_id.as_u32())
    }

    pub fn placeholder_to_expn_id(self) -> LocalExpnId {
        LocalExpnId::from_u32(self.as_u32())
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6 */

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_u32(), f)
    }
}