macro_rules! deps {
    () => {
        FileRangeWrapper!();
        InFileWrapper!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < FileId : Copy , N : AstNode > InFileWrapper < FileId , N > { pub fn syntax (& self) -> InFileWrapper < FileId , & SyntaxNode > { self . with_value (self . value . syntax ()) } pub fn node_file_range (& self) -> FileRangeWrapper < FileId > { FileRangeWrapper { file_id : self . file_id , range : self . value . syntax () . text_range () } } }
    };
}

impl_94!()