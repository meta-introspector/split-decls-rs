macro_rules! deps {
    () => {
        ProgressId!();
        Id!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: IndexObjects => * b"IWIO" , ProgressId :: DecompressedBytes => * b"IWDB" , ProgressId :: ResolveObjects => * b"IWRO" , ProgressId :: DecodedBytes => * b"IWDB" , ProgressId :: IndexBytesWritten => * b"IWBW" , } } }
    };
}

impl_279!();