macro_rules! deps {
    () => {
        Sink!();
        SinkExt!();
    };
}

macro_rules! impl_988 {
    () => {
        deps!();
        impl < T : ? Sized , Item > SinkExt < Item > for T where T : Sink < Item > { }
    };
}

impl_988!()