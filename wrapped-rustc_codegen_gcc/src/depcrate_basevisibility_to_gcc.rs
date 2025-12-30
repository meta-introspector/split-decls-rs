// Generated macro for visibility_to_gcc (function)
macro_rules! Depcrate_basevisibility_to_gcc {
() => {
// Module: crate::base
// Provides: {"visibility_to_gcc"}
// Dependencies: {}
# [cfg (feature = "master")] pub fn visibility_to_gcc (visibility : Visibility) -> gccjit :: Visibility { match visibility { Visibility :: Default => gccjit :: Visibility :: Default , Visibility :: Hidden => gccjit :: Visibility :: Hidden , Visibility :: Protected => gccjit :: Visibility :: Protected , } }
};
}
