// Generated macro for impl_1276 (impl)
macro_rules! Depcrate_iter_while_someimpl_1276 {
() => {
// Module: crate::iter::while_some
// Provides: {"impl_1276"}
// Dependencies: {}
impl < 'f , T , C > Consumer < Option < T > > for WhileSomeConsumer < 'f , C > where C : Consumer < T > , T : Send , { type Folder = WhileSomeFolder < 'f , C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (WhileSomeConsumer { base : left , .. self } , WhileSomeConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { WhileSomeFolder { base : self . base . into_folder () , full : self . full , } } fn full (& self) -> bool { self . full . load (Ordering :: Relaxed) || self . base . full () } }
};
}
