// Generated macro for Branch (struct)
macro_rules! Depcrate_branchBranch {
() => {
// Module: crate::branch
// Provides: {"Branch"}
// Dependencies: {}
# [doc = " A structure to represent a git [branch][1]"] # [doc = ""] # [doc = " A branch is currently just a wrapper to an underlying `Reference`. The"] # [doc = " reference can be accessed through the `get` and `into_reference` methods."] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Branching-What-a-Branch-Is"] pub struct Branch < 'repo > { inner : Reference < 'repo > , }
};
}
