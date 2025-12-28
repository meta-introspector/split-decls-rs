macro_rules! deps {
    () => {
        HasSource!();
        Label!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl HasSource for Label { type Ast = ast :: Label ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let (_body , source_map) = db . body_with_source_map (self . parent) ; let src = source_map . label_syntax (self . label_id) ; let root = src . file_syntax (db) ; Some (src . map (| ast | ast . to_node (& root))) } }
    };
}

impl_67!()