macro_rules! deps {
    () => {
        InFileWrapper!();
        ExpandDatabase!();
        InRealFile!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T > InFileWrapper < span :: FileId , T > { pub fn with_edition (self , db : & dyn ExpandDatabase , edition : span :: Edition) -> InRealFile < T > { InRealFile { file_id : EditionedFileId :: new (db , self . file_id , edition) , value : self . value } } }
    };
}

impl_70!();