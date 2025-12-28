macro_rules! deps {
    () => {
        ErasedAstId!();
        ExpandDatabase!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl ErasedAstId { pub fn to_range (& self , db : & dyn ExpandDatabase) -> TextRange { self . to_ptr (db) . text_range () } pub fn to_ptr (& self , db : & dyn ExpandDatabase) -> SyntaxNodePtr { db . ast_id_map (self . file_id) . get_erased (self . value) } }
    };
}

impl_80!()