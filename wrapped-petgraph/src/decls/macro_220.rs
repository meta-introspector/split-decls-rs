macro_rules! macro_220 {
    () => {
        trait_template ! { # [doc = " Access node and edge weights (associated data)."] # [allow (clippy :: needless_arbitrary_self_type)] pub trait DataMap : Data { @ section self fn node_weight (self : & Self , id : Self :: NodeId) -> Option <& Self :: NodeWeight >; fn edge_weight (self : & Self , id : Self :: EdgeId) -> Option <& Self :: EdgeWeight >; } }
    };
}

macro_220!()