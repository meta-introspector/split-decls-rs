// Generated macro for impl_17 (impl)
macro_rules! Depcrate_entryimpl_17 {
() => {
// Module: crate::entry
// Provides: {"impl_17"}
// Dependencies: {}
impl From < std :: fs :: FileType > for Kind { fn from (value : FileType) -> Self { if value . is_dir () { Kind :: Directory } else if value . is_symlink () { Kind :: Symlink } else if value . is_file () { Kind :: File } else { Kind :: Untrackable } } }
};
}
