// Generated macro for impl_534 (impl)
macro_rules! Depcrateimpl_534 {
() => {
// Module: crate
// Provides: {"impl_534"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl std :: error :: Error for ToCompactStringError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { ToCompactStringError :: Reserve (err) => Some (err) , ToCompactStringError :: Fmt (err) => Some (err) , } } }
};
}
