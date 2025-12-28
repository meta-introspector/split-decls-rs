macro_rules! deps {
    () => {
        SumConsumer!();
    };
}

macro_rules! impl_863 {
    () => {
        deps!();
        unsafe impl < S : Send > Send for SumConsumer < S > { }
    };
}

impl_863!();