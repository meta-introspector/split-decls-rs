macro_rules! deps {
    () => {
        EdgeReference!();
        EdgeReferences!();
        IndexType!();
        Row!();
        SomeIter!();
        WSuc!();
    };
}

macro_rules! macro_308 {
    () => {
        deps!();
        iterator_wrap ! { impl (Iterator) for # [doc = " An iterator over the [`EdgeReference`] of all the edges of the graph."] struct EdgeReferences <'a , E , Ix > where { Ix : IndexType } item : EdgeReference <'a , E , Ix >, iter : core :: iter :: FlatMap < core :: iter :: Enumerate < core :: slice :: Iter <'a , Row < E , Ix >> >, SomeIter <'a , E , Ix >, fn ((usize , &'a Vec < WSuc < E , Ix >>)) -> SomeIter <'a , E , Ix >, >, }
    };
}

macro_308!()