// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
# [doc = " Transformation"] impl Url { # [doc = " Turn a file URL like `file://relative` into `file:///root/relative`, hence it assures the URL's path component is absolute, using"] # [doc = " `current_dir` if necessary."] pub fn canonicalized (& self , current_dir : & std :: path :: Path) -> Result < Self , gix_path :: realpath :: Error > { let mut res = self . clone () ; res . canonicalize (current_dir) ? ; Ok (res) } }
};
}
