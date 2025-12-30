// Generated macro for impl_46 (impl)
macro_rules! Depcrate_file_readerimpl_46 {
() => {
// Module: crate::file_reader
// Provides: {"impl_46"}
// Dependencies: {}
impl From < u16 > for ReadyState { fn from (val : u16) -> Self { match val { 0 => ReadyState :: Empty , 1 => ReadyState :: Loading , 2 => ReadyState :: Done , _ => throw_str ("got invalid value for FileReader.readyState") , } } }
};
}
