// Generated macro for ItemListKind (enum)
macro_rules! Depcrate_contextItemListKind {
() => {
// Module: crate::context
// Provides: {"ItemListKind"}
// Dependencies: {}
# [doc = " The kind of item list a [`PathKind::Item`] belongs to."] # [derive (Debug , PartialEq , Eq)] pub (crate) enum ItemListKind { SourceFile , Module , Impl , TraitImpl (Option < ast :: Impl >) , Trait , ExternBlock { is_unsafe : bool } , }
};
}
