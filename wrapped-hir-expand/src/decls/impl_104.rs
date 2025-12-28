macro_rules! deps {
    () => {
        InFile!();
        InRealFile!();
        HirFileId!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T > InFile < T > { pub fn into_real_file (self) -> Result < InRealFile < T > , InFile < T > > { match self . file_id { HirFileId :: FileId (file_id) => Ok (InRealFile { file_id , value : self . value }) , HirFileId :: MacroFile (_) => Err (self) , } } }
    };
}

impl_104!()