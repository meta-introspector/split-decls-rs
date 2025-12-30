// Generated macro for impl_1206 (impl)
macro_rules! Depcrate_iter_unzipimpl_1206 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1206"}
// Dependencies: {}
impl < P , L , R , T > UnzipOp < T > for PartitionMap < P > where P : Fn (T) -> Either < L , R > + Sync + Send , L : Send , R : Send , { type Left = L ; type Right = R ; fn consume < FA , FB > (& self , item : T , left : FA , right : FB) -> (FA , FB) where FA : Folder < L > , FB : Folder < R > , { match (self . predicate) (item) { Either :: Left (item) => (left . consume (item) , right) , Either :: Right (item) => (left , right . consume (item)) , } } }
};
}
