macro_rules! deps {
    () => {
        FilePositionWrapper!();
        FilePosition!();
        ExpandDatabase!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl FilePositionWrapper < span :: FileId > { pub fn with_edition (self , db : & dyn ExpandDatabase , edition : span :: Edition) -> FilePosition { FilePositionWrapper { file_id : EditionedFileId :: new (db , self . file_id , edition) , offset : self . offset , } } }
    };
}

impl_68!()