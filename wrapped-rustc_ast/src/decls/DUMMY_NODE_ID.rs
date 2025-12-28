macro_rules! DUMMY_NODE_ID {
    () => {
        # [doc = " When parsing and at the beginning of doing expansions, we initially give all AST nodes"] # [doc = " this dummy AST [`NodeId`]. Then, during a later phase of expansion, we renumber them"] # [doc = " to have small, positive IDs."] pub const DUMMY_NODE_ID : NodeId = NodeId :: MAX ;
    };
}

DUMMY_NODE_ID!()