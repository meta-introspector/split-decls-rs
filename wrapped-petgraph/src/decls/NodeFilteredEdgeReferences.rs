macro_rules! NodeFilteredEdgeReferences {
    () => {
        # [doc = " A filtered edges iterator."] # [derive (Debug , Clone)] pub struct NodeFilteredEdgeReferences < 'a , G , I , F : 'a > { graph : PhantomData < G > , iter : I , f : & 'a F , }
    };
}

NodeFilteredEdgeReferences!()