macro_rules! GraphIndex {
    () => {
        # [doc = " A `GraphIndex` is a node or edge index."] pub trait GraphIndex : Copy { # [doc (hidden)] fn index (& self) -> usize ; # [doc (hidden)] fn is_node_index () -> bool ; }
    };
}

GraphIndex!();