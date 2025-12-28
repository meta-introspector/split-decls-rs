macro_rules! deps {
    () => {
        InFileWrapper!();
        FileIdToSyntax!();
        ExpandDatabase!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [allow (private_bounds)] impl < FileId : FileIdToSyntax , N : AstNode > InFileWrapper < FileId , AstPtr < N > > { pub fn to_node (& self , db : & dyn ExpandDatabase) -> N { self . value . to_node (& self . file_syntax (db)) } }
    };
}

impl_93!();