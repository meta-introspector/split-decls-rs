// Generated macro for impl_3513 (impl)
macro_rules! Depcrate_loops_explicit_iter_loopimpl_3513 {
() => {
// Module: crate::loops::explicit_iter_loop
// Provides: {"impl_3513"}
// Dependencies: {}
impl AdjustKind { fn borrow (mutbl : Mutability) -> Self { match mutbl { Mutability :: Not => Self :: Borrow , Mutability :: Mut => Self :: BorrowMut , } } fn auto_borrow (mutbl : AutoBorrowMutability) -> Self { match mutbl { AutoBorrowMutability :: Not => Self :: Borrow , AutoBorrowMutability :: Mut { .. } => Self :: BorrowMut , } } fn reborrow (mutbl : Mutability) -> Self { match mutbl { Mutability :: Not => Self :: Reborrow , Mutability :: Mut => Self :: ReborrowMut , } } fn auto_reborrow (mutbl : AutoBorrowMutability) -> Self { match mutbl { AutoBorrowMutability :: Not => Self :: Reborrow , AutoBorrowMutability :: Mut { .. } => Self :: ReborrowMut , } } fn display (self) -> & 'static str { match self { Self :: None => "" , Self :: Borrow => "&" , Self :: BorrowMut => "&mut " , Self :: Deref => "*" , Self :: Reborrow => "&*" , Self :: ReborrowMut => "&mut *" , } } }
};
}
