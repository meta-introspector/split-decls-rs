macro_rules! deps {
    () => {
        ProgressId!();
        Id!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: FromPathsCollectingEntries => * b"MPCE" , ProgressId :: BytesWritten => * b"MPBW" , } } }
    };
}

impl_293!();