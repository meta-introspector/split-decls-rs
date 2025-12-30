// Generated macro for Refspec (struct)
macro_rules! Depcrate_refspecRefspec {
() => {
// Module: crate::refspec
// Provides: {"Refspec"}
// Dependencies: {}
# [doc = " A structure to represent a git [refspec][1]."] # [doc = ""] # [doc = " Refspecs are currently mainly accessed/created through a `Remote`."] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-The-Refspec"] pub struct Refspec < 'remote > { raw : * const raw :: git_refspec , _marker : marker :: PhantomData < & 'remote raw :: git_remote > , }
};
}
