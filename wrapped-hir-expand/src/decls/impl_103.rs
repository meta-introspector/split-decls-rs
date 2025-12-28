macro_rules! deps {
    () => {
        FileRange!();
        InRealFile!();
        HirFileId!();
        ExpandDatabase!();
        Attr!();
        MacroKind!();
        InFile!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < N : AstNode > InFile < N > { pub fn original_ast_node_rooted (self , db : & dyn db :: ExpandDatabase) -> Option < InRealFile < N > > { let file_id = match self . file_id { HirFileId :: FileId (file_id) => { return Some (InRealFile { file_id , value : self . value }) ; } HirFileId :: MacroFile (m) => m , } ; if ! matches ! (file_id . kind (db) , MacroKind :: Attr | MacroKind :: AttrBuiltIn) { return None ; } let FileRange { file_id : editioned_file_id , range } = map_node_range_up_rooted (db , & db . expansion_span_map (file_id) , self . value . syntax () . text_range () ,) ? ; let anc = db . parse (editioned_file_id) . syntax_node () . covering_element (range) ; let value = anc . ancestors () . find_map (N :: cast) ? ; Some (InRealFile :: new (editioned_file_id , value)) } }
    };
}

impl_103!()