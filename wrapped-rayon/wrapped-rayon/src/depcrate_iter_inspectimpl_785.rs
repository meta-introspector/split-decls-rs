// Generated macro for impl_785 (impl)
macro_rules! Depcrate_iter_inspectimpl_785 {
() => {
// Module: crate::iter::inspect
// Provides: {"impl_785"}
// Dependencies: {}
impl < 'f , T , C , F > Consumer < T > for InspectConsumer < 'f , C , F > where C : Consumer < T > , F : Fn (& T) + Sync , { type Folder = InspectFolder < 'f , C :: Folder , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (InspectConsumer :: new (left , self . inspect_op) , InspectConsumer :: new (right , self . inspect_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { InspectFolder { base : self . base . into_folder () , inspect_op : self . inspect_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
