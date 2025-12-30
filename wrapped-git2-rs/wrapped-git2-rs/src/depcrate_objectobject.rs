// Generated macro for Object (struct)
macro_rules! Depcrate_objectObject {
() => {
// Module: crate::object
// Provides: {"Object"}
// Dependencies: {}
# [doc = " A structure to represent a git [object][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects"] pub struct Object < 'repo > { raw : * mut raw :: git_object , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
