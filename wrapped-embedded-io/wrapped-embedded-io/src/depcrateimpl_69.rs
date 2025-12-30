// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl From < ReadExactError < std :: io :: Error > > for std :: io :: Error { fn from (err : ReadExactError < std :: io :: Error >) -> Self { match err { ReadExactError :: UnexpectedEof => std :: io :: Error :: new (std :: io :: ErrorKind :: UnexpectedEof , "UnexpectedEof" . to_owned () ,) , ReadExactError :: Other (e) => std :: io :: Error :: new (e . kind () , format ! ("{e:?}")) , } } }
};
}
