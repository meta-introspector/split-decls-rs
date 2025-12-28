macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl RawStream for std :: io :: StdoutLock < '_ > { }
    };
}

impl_47!()