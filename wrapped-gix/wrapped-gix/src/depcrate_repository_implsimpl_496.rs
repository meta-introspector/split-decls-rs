// Generated macro for impl_496 (impl)
macro_rules! Depcrate_repository_implsimpl_496 {
() => {
// Module: crate::repository::impls
// Provides: {"impl_496"}
// Dependencies: {}
impl std :: fmt :: Debug for crate :: Repository { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Repository") . field ("kind" , & self . kind ()) . field ("git_dir" , & self . git_dir ()) . field ("workdir" , & self . workdir ()) . finish () } }
};
}
