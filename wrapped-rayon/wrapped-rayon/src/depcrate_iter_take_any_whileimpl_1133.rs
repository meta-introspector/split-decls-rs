// Generated macro for impl_1133 (impl)
macro_rules! Depcrate_iter_take_any_whileimpl_1133 {
() => {
// Module: crate::iter::take_any_while
// Provides: {"impl_1133"}
// Dependencies: {}
impl < 'p , T , C , P > Consumer < T > for TakeAnyWhileConsumer < 'p , C , P > where C : Consumer < T > , P : Fn (& T) -> bool + Sync , { type Folder = TakeAnyWhileFolder < 'p , C :: Folder , P > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (TakeAnyWhileConsumer { base : left , .. self } , TakeAnyWhileConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { TakeAnyWhileFolder { base : self . base . into_folder () , predicate : self . predicate , taking : self . taking , } } fn full (& self) -> bool { ! self . taking . load (Ordering :: Relaxed) || self . base . full () } }
};
}
