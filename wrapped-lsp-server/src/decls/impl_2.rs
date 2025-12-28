macro_rules! deps {
    () => {
        ProtocolError!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl std :: error :: Error for ProtocolError { }
    };
}

impl_2!()