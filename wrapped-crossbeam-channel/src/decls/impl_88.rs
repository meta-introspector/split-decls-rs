macro_rules! deps {
    () => {
        RecvError!();
        TryRecvError!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl From < RecvError > for TryRecvError { fn from (err : RecvError) -> Self { match err { RecvError => Self :: Disconnected , } } }
    };
}

impl_88!()