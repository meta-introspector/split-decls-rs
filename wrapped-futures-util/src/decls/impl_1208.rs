macro_rules! deps {
    () => {
        ReadHalf!();
        WriteHalf!();
        ReuniteError!();
    };
}

macro_rules! impl_1208 {
    () => {
        deps!();
        impl < T : Unpin > WriteHalf < T > { # [doc = " Attempts to put the two \"halves\" of a split `AsyncRead + AsyncWrite` back"] # [doc = " together. Succeeds only if the `ReadHalf<T>` and `WriteHalf<T>` are"] # [doc = " a matching pair originating from the same call to `AsyncReadExt::split`."] pub fn reunite (self , other : ReadHalf < T >) -> Result < T , ReuniteError < T > > { other . reunite (self) } }
    };
}

impl_1208!();