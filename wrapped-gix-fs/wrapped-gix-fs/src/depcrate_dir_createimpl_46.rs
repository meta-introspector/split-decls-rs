// Generated macro for impl_46 (impl)
macro_rules! Depcrate_dir_createimpl_46 {
() => {
// Module: crate::dir::create
// Provides: {"impl_46"}
// Dependencies: {}
# [doc = " Construction"] impl < 'a > Iter < 'a > { # [doc = " Create a new instance that creates `target` when iterated with the default amount of [`Retries`]."] pub fn new (target : & 'a Path) -> Self { Self :: new_with_retries (target , Default :: default ()) } # [doc = " Create a new instance that creates `target` when iterated with the specified amount of `retries`."] pub fn new_with_retries (target : & 'a Path , retries : Retries) -> Self { Iter { cursors : vec ! [target] , original_retries : retries , retries , state : State :: SearchingUpwardsForExistingDirectory , } } }
};
}
