macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl RawStream for dyn std :: io :: Write + Send + Sync { }
    };
}

impl_52!();