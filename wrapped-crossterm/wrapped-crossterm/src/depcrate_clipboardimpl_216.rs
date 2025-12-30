// Generated macro for impl_216 (impl)
macro_rules! Depcrate_clipboardimpl_216 {
() => {
// Module: crate::clipboard
// Provides: {"impl_216"}
// Dependencies: {}
impl From < char > for ClipboardType { fn from (value : char) -> Self { match value { 'c' => ClipboardType :: Clipboard , 'p' => ClipboardType :: Primary , other => ClipboardType :: Other (other) , } } }
};
}
