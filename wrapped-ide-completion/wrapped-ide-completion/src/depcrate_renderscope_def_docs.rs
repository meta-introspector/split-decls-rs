// Generated macro for scope_def_docs (function)
macro_rules! Depcrate_renderscope_def_docs {
() => {
// Module: crate::render
// Provides: {"scope_def_docs"}
// Dependencies: {}
fn scope_def_docs (db : & RootDatabase , resolution : ScopeDef) -> Option < Documentation > { use hir :: ModuleDef :: * ; match resolution { ScopeDef :: ModuleDef (Module (it)) => it . docs (db) , ScopeDef :: ModuleDef (Adt (it)) => it . docs (db) , ScopeDef :: ModuleDef (Variant (it)) => it . docs (db) , ScopeDef :: ModuleDef (Const (it)) => it . docs (db) , ScopeDef :: ModuleDef (Static (it)) => it . docs (db) , ScopeDef :: ModuleDef (Trait (it)) => it . docs (db) , ScopeDef :: ModuleDef (TypeAlias (it)) => it . docs (db) , _ => None , } }
};
}
