macro_rules! deps {
    () => {
        Id!();
        ProgressId!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: HashPackDataBytes => * b"PTHP" , ProgressId :: HashPackIndexBytes => * b"PTHI" , ProgressId :: CollectSortedIndexEntries => * b"PTCE" , ProgressId :: TreeFromOffsetsObjects => * b"PTDI" , ProgressId :: DecodedObjects => * b"PTRO" , ProgressId :: DecodedBytes => * b"PTDB" , } } }
    };
}

impl_239!()