// Generated macro for impl_481 (impl)
macro_rules! Depcrate_iter_copiedimpl_481 {
() => {
// Module: crate::iter::copied
// Provides: {"impl_481"}
// Dependencies: {}
impl < 'a , T , C > Consumer < & 'a T > for CopiedConsumer < C > where C : Consumer < T > , T : 'a + Copy , { type Folder = CopiedFolder < C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (CopiedConsumer :: new (left) , CopiedConsumer :: new (right) , reducer ,) } fn into_folder (self) -> Self :: Folder { CopiedFolder { base : self . base . into_folder () , } } fn full (& self) -> bool { self . base . full () } }
};
}
