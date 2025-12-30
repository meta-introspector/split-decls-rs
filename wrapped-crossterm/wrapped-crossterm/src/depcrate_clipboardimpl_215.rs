// Generated macro for impl_215 (impl)
macro_rules! Depcrate_clipboardimpl_215 {
() => {
// Module: crate::clipboard
// Provides: {"impl_215"}
// Dependencies: {}
impl From < & ClipboardType > for char { fn from (val : & ClipboardType) -> Self { match val { ClipboardType :: Clipboard => 'c' , ClipboardType :: Primary => 'p' , ClipboardType :: Other (other) => * other , } } }
};
}
