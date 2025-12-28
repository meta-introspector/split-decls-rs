macro_rules! FilterNode {
    () => {
        # [doc = " A graph filter for nodes."] pub trait FilterNode < N > { # [doc = " Return true to have the node be part of the graph"] fn include_node (& self , node : N) -> bool ; }
    };
}

FilterNode!()