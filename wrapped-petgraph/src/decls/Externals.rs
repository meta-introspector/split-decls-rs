macro_rules! deps {
    () => {
        IndexType!();
        Direction!();
        Node!();
        DefaultIx!();
    };
}

macro_rules! Externals {
    () => {
        deps!();
        # [doc = " An iterator over either the nodes without edges to them or from them."] # [derive (Debug , Clone)] pub struct Externals < 'a , N : 'a , Ty , Ix : IndexType = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Node < Option < N > , Ix > > > , dir : Direction , ty : PhantomData < Ty > , }
    };
}

Externals!();