macro_rules! deps {
    () => {
        Graph!();
        ParseFromDot!();
        DotAttrList!();
        DotNodeWeight!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl < 'a > ParseFromDot < 'a > for crate :: graph :: Graph < DotNodeWeight < 'a > , DotAttrList < 'a > > { }
    };
}

impl_586!();