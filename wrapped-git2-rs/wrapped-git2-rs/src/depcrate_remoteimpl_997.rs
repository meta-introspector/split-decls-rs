// Generated macro for impl_997 (impl)
macro_rules! Depcrate_remoteimpl_997 {
() => {
// Module: crate::remote
// Provides: {"impl_997"}
// Dependencies: {}
impl < 'repo > Clone for Remote < 'repo > { fn clone (& self) -> Remote < 'repo > { let mut ret = ptr :: null_mut () ; let rc = unsafe { call ! (raw :: git_remote_dup (& mut ret , self . raw)) } ; assert_eq ! (rc , 0) ; Remote { raw : ret , _marker : marker :: PhantomData , } } }
};
}
