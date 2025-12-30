// Generated macro for impl_15 (impl)
macro_rules! Depcrate_joinimpl_15 {
() => {
// Module: crate::join
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'me , Tuple : Ord > JoinInput < 'me , Tuple > for & 'me Variable < Tuple > { type RecentTuples = Ref < 'me , [Tuple] > ; type StableTuples = Ref < 'me , [Relation < Tuple >] > ; fn recent (self) -> Self :: RecentTuples { Ref :: map (self . recent . borrow () , | r | & r . elements [..]) } fn stable (self) -> Self :: StableTuples { Ref :: map (self . stable . borrow () , | v | & v [..]) } }
};
}
