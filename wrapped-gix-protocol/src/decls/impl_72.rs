macro_rules! deps {
    () => {
        ProgressId!();
        RemoteProgress!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: RemoteProgress => * b"FERP" , } } }
    };
}

impl_72!();