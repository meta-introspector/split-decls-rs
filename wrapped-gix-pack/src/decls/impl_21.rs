macro_rules! deps {
    () => {
        ProgressId!();
        Id!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: ReadPackBytes => * b"BWRB" , ProgressId :: IndexingSteps (_) => * b"BWCI" , } } }
    };
}

impl_21!();