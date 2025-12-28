macro_rules! deps {
    () => {
        BuiltCrateData!();
        EditionedFileId!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl BuiltCrateData { pub fn root_file_id (& self , db : & dyn salsa :: Database) -> EditionedFileId { EditionedFileId :: new (db , self . root_file_id , self . edition) } }
    };
}

impl_53!()