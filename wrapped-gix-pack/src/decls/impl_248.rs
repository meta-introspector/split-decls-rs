macro_rules! deps {
    () => {
        Id!();
        ProgressId!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: HashPackDataBytes => * b"PTHP" , ProgressId :: HashPackIndexBytes => * b"PTHI" , ProgressId :: CollectSortedIndexEntries => * b"PTCE" , ProgressId :: DecodedObjects => * b"PTRO" , } } }
    };
}

impl_248!()