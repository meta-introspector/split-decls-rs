macro_rules! deps {
    () => {
        InFileWrapper!();
        FileRangeWrapper!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < FileId : Copy , SN : Borrow < SyntaxNode > > InFileWrapper < FileId , SN > { pub fn file_range (& self) -> FileRangeWrapper < FileId > { FileRangeWrapper { file_id : self . file_id , range : self . value . borrow () . text_range () } } }
    };
}

impl_96!()