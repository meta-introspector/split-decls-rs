macro_rules! deps {
    () => {
        Graph!();
        NodeTrait!();
        GraphMap!();
        EdgeType!();
    };
}

macro_rules! impl_885 {
    () => {
        deps!();
        # [cfg (feature = "serde-1")] impl < N , E , Ty , S > serde :: Serialize for GraphMap < N , E , Ty , S > where Ty : EdgeType , N : NodeTrait + serde :: Serialize , E : serde :: Serialize , S : BuildHasher , Self : Clone , { # [doc = " Serializes the given `GraphMap` into the same format as the standard"] # [doc = " `Graph`. Needs feature `serde-1`."] # [doc = ""] # [doc = " Note: the graph has to be `Clone` for this to work."] fn serialize < Ser > (& self , serializer : Ser) -> Result < Ser :: Ok , Ser :: Error > where Ser : serde :: Serializer , { let cloned_graph : GraphMap < N , E , Ty , S > = GraphMap :: clone (self) ; let equivalent_graph : Graph < N , E , Ty , u32 > = cloned_graph . into_graph () ; equivalent_graph . serialize (serializer) } }
    };
}

impl_885!()