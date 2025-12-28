macro_rules! scope_def_docs {
    () => {
        fn scope_def_docs (db : & RootDatabase , resolution : ScopeDef) -> Option < Documentation > { use hir :: ModuleDef :: * ; match resolution { ScopeDef :: ModuleDef (Module (it)) => it . docs (db) , ScopeDef :: ModuleDef (Adt (it)) => it . docs (db) , ScopeDef :: ModuleDef (Variant (it)) => it . docs (db) , ScopeDef :: ModuleDef (Const (it)) => it . docs (db) , ScopeDef :: ModuleDef (Static (it)) => it . docs (db) , ScopeDef :: ModuleDef (Trait (it)) => it . docs (db) , ScopeDef :: ModuleDef (TypeAlias (it)) => it . docs (db) , _ => None , } }
    };
}

scope_def_docs!();