macro_rules! deps {
    () => {
        EdgeType!();
        NodeTrait!();
        GraphMap!();
        Graph!();
    };
}

macro_rules! impl_886 {
    () => {
        deps!();
        # [cfg (feature = "serde-1")] impl < 'de , N , E , Ty , S > serde :: Deserialize < 'de > for GraphMap < N , E , Ty , S > where Ty : EdgeType , N : NodeTrait + serde :: Deserialize < 'de > , E : Clone + serde :: Deserialize < 'de > , S : BuildHasher + Default , { # [doc = " Deserializes into a new `GraphMap` from the same format as the standard"] # [doc = " `Graph`. Needs feature `serde-1`."] # [doc = ""] # [doc = " **Warning**: When deserializing a graph that was not originally a `GraphMap`,"] # [doc = " the restrictions from [`from_graph`](#method.from_graph) apply."] # [doc = ""] # [doc = " Note: The edge weights have to be `Clone` for this to work."] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let equivalent_graph : Graph < N , E , Ty , u32 > = Graph :: deserialize (deserializer) ? ; Ok (GraphMap :: from_graph (equivalent_graph)) } }
    };
}

impl_886!();