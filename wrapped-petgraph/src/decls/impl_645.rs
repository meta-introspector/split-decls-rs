macro_rules! deps {
    () => {
        FromDeserialized!();
        Graph!();
        DeserGraph!();
        EdgeType!();
        IndexType!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > FromDeserialized for Graph < N , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { type Input = DeserGraph < N , E , Ix > ; fn from_deserialized < E2 > (input : Self :: Input) -> Result < Self , E2 > where E2 : Error , { let ty = PhantomData :: < Ty > :: from_deserialized (input . edge_property) ? ; let nodes = input . nodes ; let edges = input . edges ; if nodes . len () >= < Ix as IndexType > :: max () . index () { Err (invalid_length_err :: < Ix , _ > ("node" , nodes . len ())) ? } if edges . len () >= < Ix as IndexType > :: max () . index () { Err (invalid_length_err :: < Ix , _ > ("edge" , edges . len ())) ? } let mut gr = Graph { nodes , edges , ty } ; let nc = gr . node_count () ; gr . link_edges () . map_err (| i | invalid_node_err (i . index () , nc)) ? ; Ok (gr) } }
    };
}

impl_645!()