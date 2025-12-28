macro_rules! deps {
    () => {
        Compat01As03Sink!();
    };
}

macro_rules! impl_1018 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , SinkItem > Unpin for Compat01As03Sink < S , SinkItem > { }
    };
}

impl_1018!()