macro_rules! deps {
    () => {
        HirFileId!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl PartialEq < HirFileId > for EditionedFileId { fn eq (& self , & other : & HirFileId) -> bool { other == HirFileId :: from (* self) } }
    };
}

impl_265!()