macro_rules! deps {
    () => {
        GraphError!();
    };
}

macro_rules! impl_673 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for GraphError { }
    };
}

impl_673!()