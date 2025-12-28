macro_rules! deps {
    () => {
        Sink!();
        FnMut1!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for FilterMap < S , Fut , F > where S : Stream + Sink < Item > , F : FnMut1 < S :: Item , Output = Fut > , Fut : Future , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_330!()