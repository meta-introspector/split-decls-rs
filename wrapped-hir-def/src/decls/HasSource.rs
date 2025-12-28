macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! HasSource {
    () => {
        deps!();
        pub trait HasSource { type Value : AstNode ; fn source (& self , db : & dyn DefDatabase) -> InFile < Self :: Value > { let InFile { file_id , value } = self . ast_ptr (db) ; InFile :: new (file_id , value . to_node (& db . parse_or_expand (file_id))) } fn ast_ptr (& self , db : & dyn DefDatabase) -> InFile < AstPtr < Self :: Value > > ; }
    };
}

HasSource!()