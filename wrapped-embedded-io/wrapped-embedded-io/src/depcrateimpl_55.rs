// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl From < std :: io :: SeekFrom > for SeekFrom { fn from (pos : std :: io :: SeekFrom) -> SeekFrom { match pos { std :: io :: SeekFrom :: Start (n) => SeekFrom :: Start (n) , std :: io :: SeekFrom :: End (n) => SeekFrom :: End (n) , std :: io :: SeekFrom :: Current (n) => SeekFrom :: Current (n) , } } }
};
}
