macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl RawStream for dyn std :: io :: Write + Send { }
    };
}

impl_51!()