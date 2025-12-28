macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < St , Item , Fc > Sink < Item > for FlattenUnorderedWithFlowController < St , Fc > where St : Stream + Sink < Item > , { type Error = St :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_544!();