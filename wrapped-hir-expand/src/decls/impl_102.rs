macro_rules! deps {
    () => {
        InFile!();
        HirFileId!();
        FileRange!();
        ExpandDatabase!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl InFile < TextRange > { pub fn original_node_file_range (self , db : & dyn db :: ExpandDatabase ,) -> (FileRange , SyntaxContext) { match self . file_id { HirFileId :: FileId (file_id) => { (FileRange { file_id , range : self . value } , SyntaxContext :: root (file_id . edition (db))) } HirFileId :: MacroFile (mac_file) => { match map_node_range_up (db , & db . expansion_span_map (mac_file) , self . value) { Some (it) => it , None => { let loc = db . lookup_intern_macro_call (mac_file) ; (loc . kind . original_call_range (db) , SyntaxContext :: root (loc . def . edition)) } } } } } pub fn original_node_file_range_rooted (self , db : & dyn db :: ExpandDatabase) -> FileRange { match self . file_id { HirFileId :: FileId (file_id) => FileRange { file_id , range : self . value } , HirFileId :: MacroFile (mac_file) => { match map_node_range_up_rooted (db , & db . expansion_span_map (mac_file) , self . value) { Some (it) => it , _ => { let loc = db . lookup_intern_macro_call (mac_file) ; loc . kind . original_call_range (db) } } } } } pub fn original_node_file_range_with_macro_call_input (self , db : & dyn db :: ExpandDatabase ,) -> FileRange { match self . file_id { HirFileId :: FileId (file_id) => FileRange { file_id , range : self . value } , HirFileId :: MacroFile (mac_file) => { match map_node_range_up_rooted (db , & db . expansion_span_map (mac_file) , self . value) { Some (it) => it , _ => { let loc = db . lookup_intern_macro_call (mac_file) ; loc . kind . original_call_range_with_input (db) } } } } } pub fn original_node_file_range_opt (self , db : & dyn db :: ExpandDatabase ,) -> Option < (FileRange , SyntaxContext) > { match self . file_id { HirFileId :: FileId (file_id) => Some ((FileRange { file_id , range : self . value } , SyntaxContext :: root (file_id . edition (db)) ,)) , HirFileId :: MacroFile (mac_file) => { map_node_range_up (db , & db . expansion_span_map (mac_file) , self . value) } } } pub fn original_node_file_range_rooted_opt (self , db : & dyn db :: ExpandDatabase ,) -> Option < FileRange > { match self . file_id { HirFileId :: FileId (file_id) => Some (FileRange { file_id , range : self . value }) , HirFileId :: MacroFile (mac_file) => { map_node_range_up_rooted (db , & db . expansion_span_map (mac_file) , self . value) } } } }
    };
}

impl_102!()