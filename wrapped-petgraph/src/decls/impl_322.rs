macro_rules! deps {
    () => {
        IndexType!();
        NodeIndex!();
        List!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        # [doc = " The adjacency matrix for **List** is a bitmap that's computed by"] # [doc = " `.adjacency_matrix()`."] impl < E , Ix > GetAdjacencyMatrix for List < E , Ix > where Ix : IndexType , { type AdjMatrix = FixedBitSet ; fn adjacency_matrix (& self) -> FixedBitSet { let n = self . node_count () ; let mut matrix = FixedBitSet :: with_capacity (n * n) ; for edge in self . edge_references () { let i = edge . source () . index () * n + edge . target () . index () ; matrix . put (i) ; } matrix } fn is_adjacent (& self , matrix : & FixedBitSet , a : NodeIndex < Ix > , b : NodeIndex < Ix >) -> bool { let n = self . node_count () ; let index = n * a . index () + b . index () ; matrix . contains (index) } }
    };
}

impl_322!();