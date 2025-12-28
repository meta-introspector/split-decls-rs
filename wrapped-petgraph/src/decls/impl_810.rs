macro_rules! deps {
    () => {
        StableGraph!();
    };
}

macro_rules! impl_810 {
    () => {
        deps!();
        # [doc = " The resulting cloned graph has the same graph indices as `self`."] impl < N , E , Ty , Ix > Clone for StableGraph < N , E , Ty , Ix > where N : Clone , E : Clone , Ix : Copy , { fn clone (& self) -> Self { StableGraph { g : self . g . clone () , node_count : self . node_count , edge_count : self . edge_count , free_node : self . free_node , free_edge : self . free_edge , } } fn clone_from (& mut self , rhs : & Self) { self . g . clone_from (& rhs . g) ; self . node_count = rhs . node_count ; self . edge_count = rhs . edge_count ; self . free_node = rhs . free_node ; self . free_edge = rhs . free_edge ; } }
    };
}

impl_810!();