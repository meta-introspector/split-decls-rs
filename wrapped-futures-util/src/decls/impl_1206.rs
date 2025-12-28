macro_rules! deps {
    () => {
        WriteHalf!();
        ReuniteError!();
        ReadHalf!();
    };
}

macro_rules! impl_1206 {
    () => {
        deps!();
        impl < T : Unpin > ReadHalf < T > { # [doc = " Attempts to put the two \"halves\" of a split `AsyncRead + AsyncWrite` back"] # [doc = " together. Succeeds only if the `ReadHalf<T>` and `WriteHalf<T>` are"] # [doc = " a matching pair originating from the same call to `AsyncReadExt::split`."] pub fn reunite (self , other : WriteHalf < T >) -> Result < T , ReuniteError < T > > { self . handle . reunite (other . handle) . map_err (| err | ReuniteError (Self { handle : err . 0 } , WriteHalf { handle : err . 1 })) } }
    };
}

impl_1206!()