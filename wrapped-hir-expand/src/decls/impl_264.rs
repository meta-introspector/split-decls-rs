macro_rules! deps {
    () => {
        HirFileId!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl PartialEq < EditionedFileId > for HirFileId { fn eq (& self , & other : & EditionedFileId) -> bool { * self == HirFileId :: from (other) } }
    };
}

impl_264!();