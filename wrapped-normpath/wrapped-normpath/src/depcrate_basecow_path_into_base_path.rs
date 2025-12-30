// Generated macro for cow_path_into_base_path (function)
macro_rules! Depcrate_basecow_path_into_base_path {
() => {
// Module: crate::base
// Provides: {"cow_path_into_base_path"}
// Dependencies: {}
fn cow_path_into_base_path (path : Cow < '_ , Path >) -> Cow < '_ , BasePath > { debug_assert ! (imp :: is_base (& path)) ; match path { Cow :: Borrowed (path) => { Cow :: Borrowed (BasePath :: from_inner (path . as_os_str ())) } Cow :: Owned (path) => Cow :: Owned (BasePathBuf (path)) , } }
};
}
