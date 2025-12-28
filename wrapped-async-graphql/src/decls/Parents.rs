macro_rules! deps {
    () => {
        QueryPathNode!();
    };
}

macro_rules! Parents {
    () => {
        deps!();
        # [doc = " An iterator over the parents of a"] # [doc = " [`QueryPathNode`](struct.QueryPathNode.html)."] # [derive (Debug , Clone)] pub struct Parents < 'a > (& 'a QueryPathNode < 'a >) ;
    };
}

Parents!();