macro_rules! deps {
    () => {
        ReadState!();
    };
}

macro_rules! impl_721 {
    () => {
        deps!();
        impl < St > IntoAsyncRead < St > where St : TryStream < Error = Error > , St :: Ok : AsRef < [u8] > , { pub (super) fn new (stream : St) -> Self { Self { stream , state : ReadState :: PendingChunk } } }
    };
}

impl_721!();