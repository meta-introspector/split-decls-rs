// Generated macro for impl_1203 (impl)
macro_rules! Depcrate_iter_unzipimpl_1203 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1203"}
// Dependencies: {}
impl < P , T > UnzipOp < T > for Partition < P > where P : Fn (& T) -> bool + Sync + Send , T : Send , { type Left = T ; type Right = T ; fn consume < FA , FB > (& self , item : T , left : FA , right : FB) -> (FA , FB) where FA : Folder < T > , FB : Folder < T > , { if (self . predicate) (& item) { (left . consume (item) , right) } else { (left , right . consume (item)) } } }
};
}
