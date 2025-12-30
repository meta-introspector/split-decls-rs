// Generated macro for impl_96 (impl)
macro_rules! Depcrate_item_typeimpl_96 {
() => {
// Module: crate::item_type
// Provides: {"impl_96"}
// Dependencies: {}
impl std :: fmt :: Display for DeriveItemKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { DeriveItemKind :: BinaryOp (op) => write ! (f , "{op}") , DeriveItemKind :: AssignOp (op) => write ! (f , "{op}Assign") , DeriveItemKind :: UnaryOp (op) => write ! (f , "{op}") , DeriveItemKind :: CompareOp (op) => write ! (f , "{op}") , DeriveItemKind :: Copy => write ! (f , "Copy") , DeriveItemKind :: Clone => write ! (f , "Clone") , DeriveItemKind :: Debug => write ! (f , "Debug") , DeriveItemKind :: Default => write ! (f , "Default") , DeriveItemKind :: Deref => write ! (f , "Deref") , DeriveItemKind :: DerefMut => write ! (f , "DerefMut") , } } }
};
}
