macro_rules! deps {
    () => {
        HirFileId!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl From < EditionedFileId > for HirFileId { # [inline] fn from (file_id : EditionedFileId) -> Self { HirFileId :: FileId (file_id) } }
    };
}

impl_261!()