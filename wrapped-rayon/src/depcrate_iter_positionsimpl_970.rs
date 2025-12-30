// Generated macro for impl_970 (impl)
macro_rules! Depcrate_iter_positionsimpl_970 {
() => {
// Module: crate::iter::positions
// Provides: {"impl_970"}
// Dependencies: {}
impl < 'p , T , C , P > Consumer < T > for PositionsConsumer < 'p , C , P > where C : Consumer < usize > , P : Fn (T) -> bool + Sync , { type Folder = PositionsFolder < 'p , C :: Folder , P > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , C :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (PositionsConsumer :: new (left , self . predicate , self . offset) , PositionsConsumer :: new (right , self . predicate , self . offset + index) , reducer ,) } fn into_folder (self) -> Self :: Folder { PositionsFolder { base : self . base . into_folder () , predicate : self . predicate , offset : self . offset , } } fn full (& self) -> bool { self . base . full () } }
};
}
