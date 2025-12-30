// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Service { # [doc = " Render this instance as string recognized by the git transport layer."] pub fn as_str (& self) -> & 'static str { match self { Service :: ReceivePack => "git-receive-pack" , Service :: UploadPack => "git-upload-pack" , } } }
};
}
