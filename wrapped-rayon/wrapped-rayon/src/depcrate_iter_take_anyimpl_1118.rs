// Generated macro for impl_1118 (impl)
macro_rules! Depcrate_iter_take_anyimpl_1118 {
() => {
// Module: crate::iter::take_any
// Provides: {"impl_1118"}
// Dependencies: {}
impl < 'f , T , C > Consumer < T > for TakeAnyConsumer < 'f , C > where C : Consumer < T > , T : Send , { type Folder = TakeAnyFolder < 'f , C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (TakeAnyConsumer { base : left , .. self } , TakeAnyConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { TakeAnyFolder { base : self . base . into_folder () , count : self . count , } } fn full (& self) -> bool { self . count . load (Ordering :: Relaxed) == 0 || self . base . full () } }
};
}
