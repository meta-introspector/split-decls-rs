// Generated macro for impl_183 (impl)
macro_rules! Depcrate_engine_customimpl_183 {
() => {
// Module: crate::engine::custom
// Provides: {"impl_183"}
// Dependencies: {}
impl PathCompleter { # [doc = " Any path is allowed"] pub fn any () -> Self { Self { filter : None , current_dir : None , stdio : false , } } # [doc = " Complete only files"] pub fn file () -> Self { Self :: any () . filter (| p | p . is_file ()) } # [doc = " Complete only directories"] pub fn dir () -> Self { Self :: any () . filter (| p | p . is_dir ()) } # [doc = " Include stdio (`-`)"] pub fn stdio (mut self) -> Self { self . stdio = true ; self } # [doc = " Select which paths should be completed"] pub fn filter (mut self , filter : impl Fn (& std :: path :: Path) -> bool + Send + Sync + 'static ,) -> Self { self . filter = Some (Box :: new (filter)) ; self } # [doc = " Override [`std::env::current_dir`]"] pub fn current_dir (mut self , path : impl Into < std :: path :: PathBuf >) -> Self { self . current_dir = Some (path . into ()) ; self } }
};
}
