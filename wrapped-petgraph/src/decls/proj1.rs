macro_rules! deps {
    () => {
        WSuc!();
        EdgeIndex!();
        EdgeReference!();
        IndexType!();
    };
}

macro_rules! proj1 {
    () => {
        deps!();
        fn proj1 < E , Ix : IndexType > (((successor_index , edge) , from) : ((usize , & WSuc < E , Ix >) , Ix) ,) -> EdgeReference < '_ , E , Ix > { let id = EdgeIndex { from , successor_index , } ; EdgeReference { id , edge } }
    };
}

proj1!()