// Generated macro for impl_11 (impl)
macro_rules! Depcrate_prjimpl_11 {
() => {
// Module: crate::prj
// Provides: {"impl_11"}
// Dependencies: {}
impl Display for Channel { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Channel :: Stable => write ! (f , "+stable") , Channel :: Beta => write ! (f , "+beta") , Channel :: Nightly => write ! (f , "+nightly") , Channel :: Custom (name) => write ! (f , "+{name}") , } } }
};
}
