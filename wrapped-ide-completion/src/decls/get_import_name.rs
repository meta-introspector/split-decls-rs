macro_rules! deps {
    () => {
        RenderContext!();
    };
}

macro_rules! get_import_name {
    () => {
        deps!();
        fn get_import_name (resolution : ScopeDef , ctx : & RenderContext < '_ > , import_edit : & LocatedImport ,) -> Option < hir :: Name > { if import_edit . item_to_import == import_edit . original_item { import_edit . import_path . segments () . last () . cloned () } else { scope_def_to_name (resolution , ctx , import_edit) } }
    };
}

get_import_name!();