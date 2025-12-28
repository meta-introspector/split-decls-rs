macro_rules! deps {
    () => {
        TryReadyChunksError!();
    };
}

macro_rules! impl_688 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T , E : fmt :: Debug + fmt :: Display > std :: error :: Error for TryReadyChunksError < T , E > { }
    };
}

impl_688!()