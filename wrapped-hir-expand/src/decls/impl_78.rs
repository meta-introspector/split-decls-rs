macro_rules! deps {
    () => {
        AstId!();
        ExpandDatabase!();
        InFile!();
        ErasedAstId!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < N : AstNode > AstId < N > { pub fn to_node (& self , db : & dyn ExpandDatabase) -> N { self . to_ptr (db) . to_node (& db . parse_or_expand (self . file_id)) } pub fn to_range (& self , db : & dyn ExpandDatabase) -> TextRange { self . to_ptr (db) . text_range () } pub fn to_in_file_node (& self , db : & dyn ExpandDatabase) -> crate :: InFile < N > { crate :: InFile :: new (self . file_id , self . to_ptr (db) . to_node (& db . parse_or_expand (self . file_id))) } pub fn to_ptr (& self , db : & dyn ExpandDatabase) -> AstPtr < N > { db . ast_id_map (self . file_id) . get (self . value) } pub fn erase (& self) -> ErasedAstId { crate :: InFile :: new (self . file_id , self . value . erase ()) } # [inline] pub fn upcast < M : AstIdNode > (self) -> AstId < M > where N : Into < M > , { self . map (| it | it . upcast ()) } }
    };
}

impl_78!();