macro_rules! deps {
    () => {
        EdgeReference!();
        WSuc!();
        RowIter!();
    };
}

macro_rules! SomeIter {
    () => {
        deps!();
        type SomeIter < 'a , E , Ix > = core :: iter :: Map < core :: iter :: Zip < core :: iter :: Enumerate < RowIter < 'a , E , Ix > > , core :: iter :: Repeat < Ix > > , fn (((usize , & 'a WSuc < E , Ix >) , Ix)) -> EdgeReference < 'a , E , Ix > , > ;
    };
}

SomeIter!();