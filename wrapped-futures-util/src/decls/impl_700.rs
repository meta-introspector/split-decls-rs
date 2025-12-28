macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_700 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , F , Item , E > Sink < Item > for TrySkipWhile < S , Fut , F > where S : TryStream + Sink < Item , Error = E > , { type Error = E ; delegate_sink ! (stream , Item) ; }
    };
}

impl_700!()