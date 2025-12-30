// Generated macro for impl_89 (impl)
macro_rules! Depcrate_relative_path_bufimpl_89 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_89"}
// Dependencies: {}
impl < P > Extend < P > for RelativePathBuf where P : AsRef < RelativePath > , { # [inline] fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = P > , { iter . into_iter () . for_each (move | p | self . push (p . as_ref ())) ; } }
};
}
