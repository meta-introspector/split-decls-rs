macro_rules! deps {
    () => {
        AstIdWithPath!();
    };
}

macro_rules! impl_746 {
    () => {
        deps!();
        impl < T : AstIdNode > AstIdWithPath < T > { fn new (file_id : HirFileId , ast_id : FileAstId < T > , path : Interned < ModPath >) -> AstIdWithPath < T > { AstIdWithPath { ast_id : AstId :: new (file_id , ast_id) , path } } }
    };
}

impl_746!();