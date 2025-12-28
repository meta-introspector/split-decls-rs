macro_rules! deps {
    () => {
        Kind!();
        Tree!();
        Entry!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl From < crate :: index :: Entry > for Entry { fn from (index_entry : crate :: index :: Entry) -> Self { Entry { index_entry , level : 0 , object_kind : gix_object :: Kind :: Tree , object_size : 0 , decompressed_size : 0 , compressed_size : 0 , } } }
    };
}

impl_242!()