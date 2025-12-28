macro_rules! deps {
    () => {
        GraphError!();
    };
}

macro_rules! impl_674 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] impl core :: error :: Error for GraphError { }
    };
}

impl_674!()