macro_rules! deps {
    () => {
        IndexType!();
        SomeIter!();
        WSuc!();
    };
}

macro_rules! proj2 {
    () => {
        deps!();
        fn proj2 < E , Ix : IndexType > ((row_index , row) : (usize , & Vec < WSuc < E , Ix > >)) -> SomeIter < '_ , E , Ix > { row . iter () . enumerate () . zip (core :: iter :: repeat (Ix :: new (row_index))) . map (proj1 as _) }
    };
}

proj2!();