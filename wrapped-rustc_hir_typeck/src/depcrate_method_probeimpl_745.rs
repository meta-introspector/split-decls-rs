// Generated macro for impl_745 (impl)
macro_rules! Depcrate_method_probeimpl_745 {
() => {
// Module: crate::method::probe
// Provides: {"impl_745"}
// Dependencies: {}
impl AutorefOrPtrAdjustment { fn get_unsize (& self) -> bool { match self { AutorefOrPtrAdjustment :: Autoref { mutbl : _ , unsize } => * unsize , AutorefOrPtrAdjustment :: ToConstPtr => false , AutorefOrPtrAdjustment :: ReborrowPin (_) => false , } } }
};
}
