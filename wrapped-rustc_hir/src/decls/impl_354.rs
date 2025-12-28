macro_rules! deps {
    () => {
        VisitorExt!();
        Visitor!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl < 'v , V : Visitor < 'v > > VisitorExt < 'v > for V { }
    };
}

impl_354!();