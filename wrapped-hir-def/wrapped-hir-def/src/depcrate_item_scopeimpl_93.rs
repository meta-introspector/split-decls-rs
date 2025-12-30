// Generated macro for impl_93 (impl)
macro_rules! Depcrate_item_scopeimpl_93 {
() => {
// Module: crate::item_scope
// Provides: {"impl_93"}
// Dependencies: {}
impl ImportOrExternCrate { pub fn import_or_glob (self) -> Option < ImportOrGlob > { match self { ImportOrExternCrate :: Import (it) => Some (ImportOrGlob :: Import (it)) , ImportOrExternCrate :: Glob (it) => Some (ImportOrGlob :: Glob (it)) , _ => None , } } pub fn import (self) -> Option < ImportId > { match self { ImportOrExternCrate :: Import (it) => Some (it) , _ => None , } } pub fn glob (self) -> Option < GlobId > { match self { ImportOrExternCrate :: Glob (id) => Some (id) , _ => None , } } pub fn use_ (self) -> Option < UseId > { match self { ImportOrExternCrate :: Glob (id) => Some (id . use_) , ImportOrExternCrate :: Import (id) => Some (id . use_) , _ => None , } } }
};
}
