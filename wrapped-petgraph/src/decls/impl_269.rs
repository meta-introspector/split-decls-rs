macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < G : Visitable + GetAdjacencyMatrix > GetAdjacencyMatrix for Acyclic < G > { type AdjMatrix = G :: AdjMatrix ; fn adjacency_matrix (& self) -> Self :: AdjMatrix { self . inner () . adjacency_matrix () } fn is_adjacent (& self , matrix : & Self :: AdjMatrix , a : Self :: NodeId , b : Self :: NodeId) -> bool { self . inner () . is_adjacent (matrix , a , b) } }
    };
}

impl_269!()