macro_rules! deps {
    () => {
        RecvTimeoutError!();
        RecvError!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl From < RecvError > for RecvTimeoutError { fn from (err : RecvError) -> Self { match err { RecvError => Self :: Disconnected , } } }
    };
}

impl_92!();