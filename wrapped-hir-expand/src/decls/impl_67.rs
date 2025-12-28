macro_rules! deps {
    () => {
        HirFilePosition!();
        FilePosition!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl From < FilePosition > for HirFilePosition { fn from (value : FilePosition) -> Self { HirFilePosition { file_id : value . file_id . into () , offset : value . offset } } }
    };
}

impl_67!()