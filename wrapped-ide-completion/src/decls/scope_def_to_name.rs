macro_rules! deps {
    () => {
        RenderContext!();
    };
}

macro_rules! scope_def_to_name {
    () => {
        deps!();
        fn scope_def_to_name (resolution : ScopeDef , ctx : & RenderContext < '_ > , import_edit : & LocatedImport ,) -> Option < hir :: Name > { Some (match resolution { ScopeDef :: ModuleDef (hir :: ModuleDef :: Function (f)) => f . name (ctx . completion . db) , ScopeDef :: ModuleDef (hir :: ModuleDef :: Const (c)) => c . name (ctx . completion . db) ? , ScopeDef :: ModuleDef (hir :: ModuleDef :: TypeAlias (t)) => t . name (ctx . completion . db) , _ => item_name (ctx . db () , import_edit . original_item) ? , }) }
    };
}

scope_def_to_name!()