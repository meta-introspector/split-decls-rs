// Generated macro for impl_286 (impl)
macro_rules! Depcrate_kindimpl_286 {
() => {
// Module: crate::kind
// Provides: {"impl_286"}
// Dependencies: {}
# [doc = " Initialization"] impl Kind { # [doc = " Parse a `Kind` from its serialized loose git objects."] pub fn from_bytes (s : & [u8]) -> Result < Kind , Error > { Ok (match s { b"tree" => Kind :: Tree , b"blob" => Kind :: Blob , b"commit" => Kind :: Commit , b"tag" => Kind :: Tag , _ => return Err (Error :: InvalidObjectKind { kind : s . into () }) , }) } }
};
}
