// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl From < SeekFrom > for std :: io :: SeekFrom { fn from (pos : SeekFrom) -> Self { match pos { SeekFrom :: Start (n) => std :: io :: SeekFrom :: Start (n) , SeekFrom :: End (n) => std :: io :: SeekFrom :: End (n) , SeekFrom :: Current (n) => std :: io :: SeekFrom :: Current (n) , } } }
};
}
