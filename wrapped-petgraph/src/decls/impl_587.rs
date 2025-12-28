macro_rules! deps {
    () => {
        DotNodeWeight!();
        DotAttrList!();
        StableGraph!();
        ParseFromDot!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < 'a > ParseFromDot < 'a > for crate :: stable_graph :: StableGraph < DotNodeWeight < 'a > , DotAttrList < 'a > > { }
    };
}

impl_587!();