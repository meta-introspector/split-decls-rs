macro_rules! EdgeFilteredEdges {
    () => {
        # [doc = " A filtered edges iterator."] # [derive (Debug , Clone)] pub struct EdgeFilteredEdges < 'a , G , I , F : 'a > { graph : PhantomData < G > , iter : I , f : & 'a F , }
    };
}

EdgeFilteredEdges!();