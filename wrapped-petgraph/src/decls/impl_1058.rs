macro_rules! deps {
    () => {
        IndexType!();
        Graph!();
        EdgeType!();
        NodeIndex!();
    };
}

macro_rules! impl_1058 {
    () => {
        deps!();
        # [doc = " The adjacency matrix for **Graph** is a bitmap that's computed by"] # [doc = " `.adjacency_matrix()`."] impl < N , E , Ty , Ix > GetAdjacencyMatrix for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type AdjMatrix = FixedBitSet ; fn adjacency_matrix (& self) -> FixedBitSet { let n = self . node_count () ; let mut matrix = FixedBitSet :: with_capacity (n * n) ; for edge in self . edge_references () { let i = edge . source () . index () * n + edge . target () . index () ; matrix . put (i) ; if ! self . is_directed () { let j = edge . source () . index () + n * edge . target () . index () ; matrix . put (j) ; } } matrix } fn is_adjacent (& self , matrix : & FixedBitSet , a : NodeIndex < Ix > , b : NodeIndex < Ix >) -> bool { let n = self . node_count () ; let index = n * a . index () + b . index () ; matrix . contains (index) } }
    };
}

impl_1058!()