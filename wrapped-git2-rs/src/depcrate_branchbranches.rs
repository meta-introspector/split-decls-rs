// Generated macro for Branches (struct)
macro_rules! Depcrate_branchBranches {
() => {
// Module: crate::branch
// Provides: {"Branches"}
// Dependencies: {}
# [doc = " An iterator over the branches inside of a repository."] pub struct Branches < 'repo > { raw : * mut raw :: git_branch_iterator , _marker : marker :: PhantomData < References < 'repo > > , }
};
}
