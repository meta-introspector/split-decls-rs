// Generated macro for impl_1200 (impl)
macro_rules! Depcrate_iter_unzipimpl_1200 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1200"}
// Dependencies: {}
impl < A : Send , B : Send > UnzipOp < (A , B) > for Unzip { type Left = A ; type Right = B ; fn consume < FA , FB > (& self , item : (A , B) , left : FA , right : FB) -> (FA , FB) where FA : Folder < A > , FB : Folder < B > , { (left . consume (item . 0) , right . consume (item . 1)) } fn indexable () -> bool { true } }
};
}
