macro_rules! deps {
    () => {
        FileRange!();
        HirFileRange!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl From < FileRange > for HirFileRange { fn from (value : FileRange) -> Self { HirFileRange { file_id : value . file_id . into () , range : value . range } } }
    };
}

impl_66!();