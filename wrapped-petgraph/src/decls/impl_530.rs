macro_rules! deps {
    () => {
        EdgeType!();
        Csr!();
        EdgeRef!();
        IndexType!();
        EdgeReferences!();
        EdgeReference!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > IntoEdgeReferences for & 'a Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeRef = EdgeReference < 'a , E , Ty , Ix > ; type EdgeReferences = EdgeReferences < 'a , E , Ty , Ix > ; fn edge_references (self) -> Self :: EdgeReferences { EdgeReferences { index : 0 , source_index : Ix :: new (0) , edge_ranges : self . row . windows (2) . enumerate () , column : & self . column , edges : & self . edges , iter : zip (& [] , & []) , ty : self . ty , } } }
    };
}

impl_530!()