macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , F , Item , E > Sink < Item > for TryTakeWhile < S , Fut , F > where S : TryStream + Sink < Item , Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
    };
}

impl_707!();