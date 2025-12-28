macro_rules! deps {
    () => {
        HasSource!();
        Module!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [doc = " NB: Module is !HasSource, because it has two source nodes at the same time:"] # [doc = " definition and declaration."] impl Module { # [doc = " Returns a node which defines this module. That is, a file or a `mod foo {}` with items."] pub fn definition_source (self , db : & dyn HirDatabase) -> InFile < ModuleSource > { let def_map = self . id . def_map (db) ; def_map [self . id . local_id] . definition_source (db) } # [doc = " Returns a node which defines this module. That is, a file or a `mod foo {}` with items."] pub fn definition_source_range (self , db : & dyn HirDatabase) -> InFile < TextRange > { let def_map = self . id . def_map (db) ; def_map [self . id . local_id] . definition_source_range (db) } pub fn definition_source_file_id (self , db : & dyn HirDatabase) -> HirFileId { let def_map = self . id . def_map (db) ; def_map [self . id . local_id] . definition_source_file_id () } pub fn is_mod_rs (self , db : & dyn HirDatabase) -> bool { let def_map = self . id . def_map (db) ; match def_map [self . id . local_id] . origin { ModuleOrigin :: File { is_mod_rs , .. } => is_mod_rs , _ => false , } } pub fn as_source_file_id (self , db : & dyn HirDatabase) -> Option < EditionedFileId > { let def_map = self . id . def_map (db) ; match def_map [self . id . local_id] . origin { ModuleOrigin :: File { definition , .. } | ModuleOrigin :: CrateRoot { definition , .. } => { Some (definition) } _ => None , } } pub fn is_inline (self , db : & dyn HirDatabase) -> bool { let def_map = self . id . def_map (db) ; def_map [self . id . local_id] . origin . is_inline () } # [doc = " Returns a node which declares this module, either a `mod foo;` or a `mod foo {}`."] # [doc = " `None` for the crate root."] pub fn declaration_source (self , db : & dyn HirDatabase) -> Option < InFile < ast :: Module > > { let def_map = self . id . def_map (db) ; def_map [self . id . local_id] . declaration_source (db) } # [doc = " Returns a text range which declares this module, either a `mod foo;` or a `mod foo {}`."] # [doc = " `None` for the crate root."] pub fn declaration_source_range (self , db : & dyn HirDatabase) -> Option < InFile < TextRange > > { let def_map = self . id . def_map (db) ; def_map [self . id . local_id] . declaration_source_range (db) } }
    };
}

impl_47!();