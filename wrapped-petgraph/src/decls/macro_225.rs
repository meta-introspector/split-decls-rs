macro_rules! macro_225 {
    () => {
        trait_template ! { # [doc = " Access node and edge weights mutably."] # [allow (clippy :: needless_arbitrary_self_type)] pub trait DataMapMut : DataMap { @ section self fn node_weight_mut (self : & mut Self , id : Self :: NodeId) -> Option <& mut Self :: NodeWeight >; fn edge_weight_mut (self : & mut Self , id : Self :: EdgeId) -> Option <& mut Self :: EdgeWeight >; } }
    };
}

macro_225!()