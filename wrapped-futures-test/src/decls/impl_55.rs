macro_rules! deps {
    () => {
        SinkTestExt!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < Item , W > SinkTestExt < Item > for W where W : Sink < Item > { }
    };
}

impl_55!()