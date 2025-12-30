// Generated macro for Submodule (struct)
macro_rules! Depcrate_submoduleSubmodule {
() => {
// Module: crate::submodule
// Provides: {"Submodule"}
// Dependencies: {}
# [doc = " A structure to represent a git [submodule][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Tools-Submodules"] pub struct Submodule < 'repo > { raw : * mut raw :: git_submodule , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
