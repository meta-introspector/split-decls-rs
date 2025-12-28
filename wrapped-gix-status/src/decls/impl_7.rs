macro_rules! deps {
    () => {
        Entry!();
        ConflictIndexEntry!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl From < & gix_index :: Entry > for ConflictIndexEntry { fn from (gix_index :: Entry { stat : _ , id , flags , mode , .. } : & gix_index :: Entry ,) -> Self { ConflictIndexEntry { id : * id , flags : * flags , mode : * mode , } } }
    };
}

impl_7!();