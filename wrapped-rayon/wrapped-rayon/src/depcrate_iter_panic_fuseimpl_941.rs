// Generated macro for impl_941 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_941 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_941"}
// Dependencies: {}
impl < 'a , T , C > Consumer < T > for PanicFuseConsumer < 'a , C > where C : Consumer < T > , { type Folder = PanicFuseFolder < 'a , C :: Folder > ; type Reducer = PanicFuseReducer < 'a , C :: Reducer > ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (PanicFuseConsumer { base : left , fuse : self . fuse . clone () , } , PanicFuseConsumer { base : right , fuse : self . fuse . clone () , } , PanicFuseReducer { base : reducer , _fuse : self . fuse , } ,) } fn into_folder (self) -> Self :: Folder { PanicFuseFolder { base : self . base . into_folder () , fuse : self . fuse , } } fn full (& self) -> bool { self . fuse . panicked () || self . base . full () } }
};
}
