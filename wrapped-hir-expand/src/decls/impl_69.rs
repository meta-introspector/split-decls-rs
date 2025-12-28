macro_rules! deps {
    () => {
        FileRangeWrapper!();
        ExpandDatabase!();
        FileRange!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl FileRangeWrapper < span :: FileId > { pub fn with_edition (self , db : & dyn ExpandDatabase , edition : span :: Edition) -> FileRange { FileRangeWrapper { file_id : EditionedFileId :: new (db , self . file_id , edition) , range : self . range , } } }
    };
}

impl_69!()