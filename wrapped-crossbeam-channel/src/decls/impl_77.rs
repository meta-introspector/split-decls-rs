macro_rules! deps {
    () => {
        SendError!();
        TrySendError!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T > From < SendError < T > > for TrySendError < T > { fn from (err : SendError < T >) -> Self { match err { SendError (t) => Self :: Disconnected (t) , } } }
    };
}

impl_77!()