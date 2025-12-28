macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl RawStream for std :: io :: StderrLock < '_ > { }
    };
}

impl_49!();