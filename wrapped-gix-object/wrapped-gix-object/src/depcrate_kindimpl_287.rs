// Generated macro for impl_287 (impl)
macro_rules! Depcrate_kindimpl_287 {
() => {
// Module: crate::kind
// Provides: {"impl_287"}
// Dependencies: {}
# [doc = " Access"] impl Kind { # [doc = " Return the name of `self` for use in serialized loose git objects."] pub fn as_bytes (& self) -> & [u8] { match self { Kind :: Tree => b"tree" , Kind :: Commit => b"commit" , Kind :: Blob => b"blob" , Kind :: Tag => b"tag" , } } # [doc = " Returns `true` if this instance is representing a commit."] pub fn is_commit (& self) -> bool { matches ! (self , Kind :: Commit) } # [doc = " Returns `true` if this instance is representing a tree."] pub fn is_tree (& self) -> bool { matches ! (self , Kind :: Tree) } # [doc = " Returns `true` if this instance is representing a tag."] pub fn is_tag (& self) -> bool { matches ! (self , Kind :: Tag) } # [doc = " Returns `true` if this instance is representing a blob."] pub fn is_blob (& self) -> bool { matches ! (self , Kind :: Blob) } }
};
}
