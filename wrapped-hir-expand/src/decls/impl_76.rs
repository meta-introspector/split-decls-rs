macro_rules! deps {
    () => {
        ExpandDatabase!();
        FileRangeWrapper!();
        FileRange!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl FileRange { # [inline] pub fn into_file_id (self , db : & dyn ExpandDatabase) -> FileRangeWrapper < FileId > { FileRangeWrapper { file_id : self . file_id . file_id (db) , range : self . range } } # [inline] pub fn file_text (self , db : & dyn ExpandDatabase) -> & triomphe :: Arc < str > { db . file_text (self . file_id . file_id (db)) . text (db) } # [inline] pub fn text (self , db : & dyn ExpandDatabase) -> & str { & self . file_text (db) [self . range] } }
    };
}

impl_76!()