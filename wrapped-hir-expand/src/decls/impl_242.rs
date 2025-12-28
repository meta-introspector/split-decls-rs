macro_rules! deps {
    () => {
        MacroCallKind!();
        InFile!();
        InRealFile!();
        ExpandDatabase!();
        MacroDefKind!();
        Attr!();
        HirFileId!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl HirFileId { pub fn edition (self , db : & dyn ExpandDatabase) -> Edition { match self { HirFileId :: FileId (file_id) => file_id . editioned_file_id (db) . edition () , HirFileId :: MacroFile (m) => db . lookup_intern_macro_call (m) . def . edition , } } pub fn original_file (self , db : & dyn ExpandDatabase) -> EditionedFileId { let mut file_id = self ; loop { match file_id { HirFileId :: FileId (id) => break id , HirFileId :: MacroFile (macro_call_id) => { file_id = db . lookup_intern_macro_call (macro_call_id) . kind . file_id () } } } } pub fn original_file_respecting_includes (mut self , db : & dyn ExpandDatabase) -> EditionedFileId { loop { match self { HirFileId :: FileId (id) => break id , HirFileId :: MacroFile (file) => { let loc = db . lookup_intern_macro_call (file) ; if loc . def . is_include () && let MacroCallKind :: FnLike { eager : Some (eager) , .. } = & loc . kind && let Ok (it) = include_input_to_file_id (db , file , & eager . arg) { break it ; } self = loc . kind . file_id () ; } } } } pub fn original_call_node (self , db : & dyn ExpandDatabase) -> Option < InRealFile < SyntaxNode > > { let mut call = db . lookup_intern_macro_call (self . macro_file () ?) . to_node (db) ; loop { match call . file_id { HirFileId :: FileId (file_id) => { break Some (InRealFile { file_id , value : call . value }) ; } HirFileId :: MacroFile (macro_call_id) => { call = db . lookup_intern_macro_call (macro_call_id) . to_node (db) ; } } } } pub fn call_node (self , db : & dyn ExpandDatabase) -> Option < InFile < SyntaxNode > > { Some (db . lookup_intern_macro_call (self . macro_file () ?) . to_node (db)) } pub fn as_builtin_derive_attr_node (& self , db : & dyn ExpandDatabase ,) -> Option < InFile < ast :: Attr > > { let macro_file = self . macro_file () ? ; let loc = db . lookup_intern_macro_call (macro_file) ; let attr = match loc . def . kind { MacroDefKind :: BuiltInDerive (..) => loc . to_node (db) , _ => return None , } ; Some (attr . with_value (ast :: Attr :: cast (attr . value . clone ()) ?)) } }
    };
}

impl_242!();