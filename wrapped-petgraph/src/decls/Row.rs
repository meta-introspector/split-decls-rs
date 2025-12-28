macro_rules! deps {
    () => {
        WSuc!();
    };
}

macro_rules! Row {
    () => {
        deps!();
        # [doc = " One row of the adjacency list."] type Row < E , Ix > = Vec < WSuc < E , Ix > > ;
    };
}

Row!()