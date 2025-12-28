macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl RawStream for dyn std :: io :: Write { }
    };
}

impl_50!();