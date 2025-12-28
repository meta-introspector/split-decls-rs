macro_rules! deps {
    () => {
        NodeIndex!();
        NeighborIterDirection!();
        IndexType!();
        EdgeType!();
        Nullable!();
        Edges!();
    };
}

macro_rules! impl_1005 {
    () => {
        deps!();
        impl < 'a , Ty : EdgeType , Null : Nullable , Ix : IndexType > Iterator for Edges < 'a , Ty , Null , Ix > { type Item = (NodeIndex < Ix > , NodeIndex < Ix > , & 'a Null :: Wrapped) ; fn next (& mut self) -> Option < Self :: Item > { use self :: NeighborIterDirection :: * ; loop { let (row , column) = (self . row , self . column) ; if row >= self . node_capacity || column >= self . node_capacity { return None ; } match self . iter_direction { Rows => self . row += 1 , Columns => self . column += 1 , } let p = to_linearized_matrix_position :: < Ty > (row , column , self . node_capacity) ; if let Some (e) = self . node_adjacencies [p] . as_ref () { let (a , b) = match self . iter_direction { Rows => (column , row) , Columns => (row , column) , } ; return Some ((NodeIndex :: new (a) , NodeIndex :: new (b) , e)) ; } } } }
    };
}

impl_1005!();