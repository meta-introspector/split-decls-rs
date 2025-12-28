macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! NodeFilteredEdges {
    () => {
        deps!();
        # [doc = " A filtered edges iterator."] # [derive (Debug , Clone)] pub struct NodeFilteredEdges < 'a , G , I , F : 'a > { graph : PhantomData < G > , include_source : bool , iter : I , f : & 'a F , dir : Direction , }
    };
}

NodeFilteredEdges!()