// Generated macro for impl_1048 (impl)
macro_rules! Depcrate_iter_skip_anyimpl_1048 {
() => {
// Module: crate::iter::skip_any
// Provides: {"impl_1048"}
// Dependencies: {}
impl < 'f , T , C > Consumer < T > for SkipAnyConsumer < 'f , C > where C : Consumer < T > , T : Send , { type Folder = SkipAnyFolder < 'f , C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (SkipAnyConsumer { base : left , .. self } , SkipAnyConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { SkipAnyFolder { base : self . base . into_folder () , count : self . count , } } fn full (& self) -> bool { self . base . full () } }
};
}
