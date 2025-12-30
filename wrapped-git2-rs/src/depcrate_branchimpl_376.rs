// Generated macro for impl_376 (impl)
macro_rules! Depcrate_branchimpl_376 {
() => {
// Module: crate::branch
// Provides: {"impl_376"}
// Dependencies: {}
impl < 'repo > Branches < 'repo > { # [doc = " Creates a new iterator from the raw pointer given."] # [doc = ""] # [doc = " This function is unsafe as it is not guaranteed that `raw` is a valid"] # [doc = " pointer."] pub unsafe fn from_raw (raw : * mut raw :: git_branch_iterator) -> Branches < 'repo > { Branches { raw , _marker : marker :: PhantomData , } } }
};
}
