macro_rules! deps {
    () => {
        Nullable!();
        EdgeType!();
        NodeIndex!();
        IndexType!();
        EdgeReferences!();
    };
}

macro_rules! impl_999 {
    () => {
        deps!();
        impl < 'a , Ty : EdgeType , Null : Nullable , Ix : IndexType > Iterator for EdgeReferences < 'a , Ty , Null , Ix > { type Item = (NodeIndex < Ix > , NodeIndex < Ix > , & 'a Null :: Wrapped) ; fn next (& mut self) -> Option < Self :: Item > { loop { let (row , column) = (self . row , self . column) ; if row >= self . node_capacity { return None ; } self . column += 1 ; let max_column_len = if ! Ty :: is_directed () { row + 1 } else { self . node_capacity } ; if self . column >= max_column_len { self . column = 0 ; self . row += 1 ; } let p = to_linearized_matrix_position :: < Ty > (row , column , self . node_capacity) ; if let Some (e) = self . node_adjacencies [p] . as_ref () { return Some ((NodeIndex :: new (row) , NodeIndex :: new (column) , e)) ; } } } }
    };
}

impl_999!()