macro_rules! deps {
    () => {
        TryChunksError!();
    };
}

macro_rules! impl_677 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T , E : fmt :: Debug + fmt :: Display > std :: error :: Error for TryChunksError < T , E > { }
    };
}

impl_677!()