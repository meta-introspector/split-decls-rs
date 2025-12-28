macro_rules! deps {
    () => {
        ModuleSource!();
        DefDatabase!();
        ModuleOrigin!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl ModuleOrigin { pub fn declaration (& self) -> Option < AstId < ast :: Module > > { match self { & ModuleOrigin :: File { declaration , declaration_tree_id , .. } => { Some (AstId :: new (declaration_tree_id . file_id () , declaration)) } & ModuleOrigin :: Inline { definition , definition_tree_id } => { Some (AstId :: new (definition_tree_id . file_id () , definition)) } ModuleOrigin :: CrateRoot { .. } | ModuleOrigin :: BlockExpr { .. } => None , } } pub fn file_id (& self) -> Option < EditionedFileId > { match self { ModuleOrigin :: File { definition , .. } | ModuleOrigin :: CrateRoot { definition } => { Some (* definition) } _ => None , } } pub fn is_inline (& self) -> bool { match self { ModuleOrigin :: Inline { .. } | ModuleOrigin :: BlockExpr { .. } => true , ModuleOrigin :: CrateRoot { .. } | ModuleOrigin :: File { .. } => false , } } # [doc = " Returns a node which defines this module."] # [doc = " That is, a file or a `mod foo {}` with items."] pub fn definition_source (& self , db : & dyn DefDatabase) -> InFile < ModuleSource > { match self { & ModuleOrigin :: File { definition : editioned_file_id , .. } | & ModuleOrigin :: CrateRoot { definition : editioned_file_id } => { let sf = db . parse (editioned_file_id) . tree () ; InFile :: new (editioned_file_id . into () , ModuleSource :: SourceFile (sf)) } & ModuleOrigin :: Inline { definition , definition_tree_id } => InFile :: new (definition_tree_id . file_id () , ModuleSource :: Module (AstId :: new (definition_tree_id . file_id () , definition) . to_node (db) ,) ,) , ModuleOrigin :: BlockExpr { block , .. } => { InFile :: new (block . file_id , ModuleSource :: BlockExpr (block . to_node (db))) } } } }
    };
}

impl_370!();