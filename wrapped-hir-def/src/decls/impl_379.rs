macro_rules! deps {
    () => {
        ModuleOrigin!();
        ModuleSource!();
        ItemScope!();
        ModuleData!();
        DefDatabase!();
        Visibility!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl ModuleData { pub (crate) fn new (origin : ModuleOrigin , visibility : Visibility) -> Self { ModuleData { origin , visibility , parent : None , children : Default :: default () , scope : ItemScope :: default () , } } # [doc = " Returns a node which defines this module. That is, a file or a `mod foo {}` with items."] pub fn definition_source (& self , db : & dyn DefDatabase) -> InFile < ModuleSource > { self . origin . definition_source (db) } # [doc = " Same as [`definition_source`] but only returns the file id to prevent parsing the ASt."] pub fn definition_source_file_id (& self) -> HirFileId { match self . origin { ModuleOrigin :: File { definition , .. } | ModuleOrigin :: CrateRoot { definition } => { definition . into () } ModuleOrigin :: Inline { definition_tree_id , .. } => definition_tree_id . file_id () , ModuleOrigin :: BlockExpr { block , .. } => block . file_id , } } pub fn definition_source_range (& self , db : & dyn DefDatabase) -> InFile < TextRange > { match & self . origin { & ModuleOrigin :: File { definition , .. } | & ModuleOrigin :: CrateRoot { definition } => { InFile :: new (definition . into () , ErasedAstId :: new (definition . into () , ROOT_ERASED_FILE_AST_ID) . to_range (db) ,) } & ModuleOrigin :: Inline { definition , definition_tree_id } => InFile :: new (definition_tree_id . file_id () , AstId :: new (definition_tree_id . file_id () , definition) . to_range (db) ,) , ModuleOrigin :: BlockExpr { block , .. } => InFile :: new (block . file_id , block . to_range (db)) , } } # [doc = " Returns a node which declares this module, either a `mod foo;` or a `mod foo {}`."] # [doc = " `None` for the crate root or block."] pub fn declaration_source (& self , db : & dyn DefDatabase) -> Option < InFile < ast :: Module > > { let decl = self . origin . declaration () ? ; let value = decl . to_node (db) ; Some (InFile { file_id : decl . file_id , value }) } # [doc = " Returns the range which declares this module, either a `mod foo;` or a `mod foo {}`."] # [doc = " `None` for the crate root or block."] pub fn declaration_source_range (& self , db : & dyn DefDatabase) -> Option < InFile < TextRange > > { let decl = self . origin . declaration () ? ; Some (InFile { file_id : decl . file_id , value : decl . to_range (db) }) } }
    };
}

impl_379!();