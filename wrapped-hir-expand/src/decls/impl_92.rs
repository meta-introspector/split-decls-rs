macro_rules! deps {
    () => {
        ExpandDatabase!();
        InFileWrapper!();
        FileIdToSyntax!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        # [allow (private_bounds)] impl < FileId : FileIdToSyntax , T > InFileWrapper < FileId , T > { pub fn file_syntax (& self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { FileIdToSyntax :: file_syntax (self . file_id , db) } }
    };
}

impl_92!()