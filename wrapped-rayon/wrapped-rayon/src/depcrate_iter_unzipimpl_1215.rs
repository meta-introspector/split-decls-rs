// Generated macro for impl_1215 (impl)
macro_rules! Depcrate_iter_unzipimpl_1215 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1215"}
// Dependencies: {}
impl < 'a , T , OP , FA , FB > Folder < T > for UnzipFolder < 'a , OP , FA , FB > where OP : UnzipOp < T > , FA : Folder < OP :: Left > , FB : Folder < OP :: Right > , { type Result = (FA :: Result , FB :: Result) ; fn consume (self , item : T) -> Self { let (left , right) = self . op . consume (item , self . left , self . right) ; UnzipFolder { op : self . op , left , right , } } fn complete (self) -> Self :: Result { (self . left . complete () , self . right . complete ()) } fn full (& self) -> bool { self . left . full () && self . right . full () } }
};
}
