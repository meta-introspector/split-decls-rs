macro_rules! deps {
    () => {
        SmartSubtransportStream!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < T : Read + Write + Send + 'static > SmartSubtransportStream for T { }
    };
}

impl_169!();