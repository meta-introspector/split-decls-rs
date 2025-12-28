macro_rules! deps {
    () => {
        HirFileId!();
        InRealFile!();
        MacroKind!();
        InFile!();
        FileRange!();
        ExpandDatabase!();
        Attr!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < SN : Borrow < SyntaxNode > > InFile < SN > { pub fn parent_ancestors_with_macros (self , db : & dyn db :: ExpandDatabase ,) -> impl Iterator < Item = InFile < SyntaxNode > > + '_ { let succ = move | node : & InFile < SyntaxNode > | match node . value . parent () { Some (parent) => Some (node . with_value (parent)) , None => db . lookup_intern_macro_call (node . file_id . macro_file () ?) . to_node_item (db) . syntax () . cloned () . map (| node | node . parent ()) . transpose () , } ; std :: iter :: successors (succ (& self . borrow () . cloned ()) , succ) } pub fn ancestors_with_macros (self , db : & dyn db :: ExpandDatabase ,) -> impl Iterator < Item = InFile < SyntaxNode > > + '_ { let succ = move | node : & InFile < SyntaxNode > | match node . value . parent () { Some (parent) => Some (node . with_value (parent)) , None => db . lookup_intern_macro_call (node . file_id . macro_file () ?) . to_node_item (db) . syntax () . cloned () . map (| node | node . parent ()) . transpose () , } ; std :: iter :: successors (Some (self . borrow () . cloned ()) , succ) } pub fn kind (& self) -> parser :: SyntaxKind { self . value . borrow () . kind () } pub fn text_range (& self) -> TextRange { self . value . borrow () . text_range () } # [doc = " Falls back to the macro call range if the node cannot be mapped up fully."] # [doc = ""] # [doc = " For attributes and derives, this will point back to the attribute only."] # [doc = " For the entire item use [`InFile::original_file_range_full`]."] pub fn original_file_range_rooted (self , db : & dyn db :: ExpandDatabase) -> FileRange { self . borrow () . map (SyntaxNode :: text_range) . original_node_file_range_rooted (db) } # [doc = " Falls back to the macro call range if the node cannot be mapped up fully."] pub fn original_file_range_with_macro_call_input (self , db : & dyn db :: ExpandDatabase ,) -> FileRange { self . borrow () . map (SyntaxNode :: text_range) . original_node_file_range_with_macro_call_input (db) } pub fn original_syntax_node_rooted (self , db : & dyn db :: ExpandDatabase ,) -> Option < InRealFile < SyntaxNode > > { let file_id = match self . file_id { HirFileId :: FileId (file_id) => { return Some (InRealFile { file_id , value : self . value . borrow () . clone () }) ; } HirFileId :: MacroFile (m) if matches ! (m . kind (db) , MacroKind :: Attr | MacroKind :: AttrBuiltIn) => { m } _ => return None , } ; let FileRange { file_id : editioned_file_id , range } = map_node_range_up_rooted (db , & db . expansion_span_map (file_id) , self . value . borrow () . text_range () ,) ? ; let kind = self . kind () ; let value = db . parse (editioned_file_id) . syntax_node () . covering_element (range) . ancestors () . take_while (| it | it . text_range () == range) . find (| it | it . kind () == kind) ? ; Some (InRealFile :: new (editioned_file_id , value)) } }
    };
}

impl_97!();