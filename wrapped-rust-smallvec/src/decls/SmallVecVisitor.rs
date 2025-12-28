macro_rules! SmallVecVisitor {
    () => {
        # [cfg (feature = "serde")] struct SmallVecVisitor < T , const N : usize > { phantom : PhantomData < T > , }
    };
}

SmallVecVisitor!()