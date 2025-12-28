macro_rules! deps {
    () => {
        VisitorNil!();
        Visitor!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl Visitor < '_ > for VisitorNil { }
    };
}

impl_292!()