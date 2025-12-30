// Generated macro for impl_1063 (impl)
macro_rules! Depcrate_iter_skip_any_whileimpl_1063 {
() => {
// Module: crate::iter::skip_any_while
// Provides: {"impl_1063"}
// Dependencies: {}
impl < 'p , T , C , P > Consumer < T > for SkipAnyWhileConsumer < 'p , C , P > where C : Consumer < T > , P : Fn (& T) -> bool + Sync , { type Folder = SkipAnyWhileFolder < 'p , C :: Folder , P > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (SkipAnyWhileConsumer { base : left , .. self } , SkipAnyWhileConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { SkipAnyWhileFolder { base : self . base . into_folder () , predicate : self . predicate , skipping : self . skipping , } } fn full (& self) -> bool { self . base . full () } }
};
}
