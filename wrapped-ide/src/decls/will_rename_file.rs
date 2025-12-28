macro_rules! will_rename_file {
    () => {
        # [doc = " Called by the client when it is about to rename a file."] pub (crate) fn will_rename_file (db : & RootDatabase , file_id : FileId , new_name_stem : & str ,) -> Option < SourceChange > { let sema = Semantics :: new (db) ; let module = sema . file_to_module_def (file_id) ? ; let def = Definition :: Module (module) ; let mut change = def . rename (& sema , new_name_stem , RenameDefinition :: Yes) . ok () ? ; change . file_system_edits . clear () ; Some (change) }
    };
}

will_rename_file!();