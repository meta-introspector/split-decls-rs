macro_rules! deps {
    () => {
        WSuc!();
    };
}

macro_rules! RowIter {
    () => {
        deps!();
        type RowIter < 'a , E , Ix > = core :: slice :: Iter < 'a , WSuc < E , Ix > > ;
    };
}

RowIter!();