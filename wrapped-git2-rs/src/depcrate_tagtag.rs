// Generated macro for Tag (struct)
macro_rules! Depcrate_tagTag {
() => {
// Module: crate::tag
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " A structure to represent a git [tag][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Basics-Tagging"] pub struct Tag < 'repo > { raw : * mut raw :: git_tag , _marker : marker :: PhantomData < Object < 'repo > > , }
};
}
