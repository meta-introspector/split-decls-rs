macro_rules! deps {
    () => {
        FilePositionWrapper!();
        FilePosition!();
        ExpandDatabase!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl FilePosition { # [inline] pub fn into_file_id (self , db : & dyn ExpandDatabase) -> FilePositionWrapper < FileId > { FilePositionWrapper { file_id : self . file_id . file_id (db) , offset : self . offset } } }
    };
}

impl_65!();