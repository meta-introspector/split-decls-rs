// Generated macro for impl_3544 (impl)
macro_rules! Depcrate_loops_explicit_into_iter_loopimpl_3544 {
() => {
// Module: crate::loops::explicit_into_iter_loop
// Provides: {"impl_3544"}
// Dependencies: {}
impl AdjustKind { fn borrow (mutbl : AutoBorrowMutability) -> Self { match mutbl { AutoBorrowMutability :: Not => Self :: Borrow , AutoBorrowMutability :: Mut { .. } => Self :: BorrowMut , } } fn reborrow (mutbl : AutoBorrowMutability) -> Self { match mutbl { AutoBorrowMutability :: Not => Self :: Reborrow , AutoBorrowMutability :: Mut { .. } => Self :: ReborrowMut , } } fn display (self) -> & 'static str { match self { Self :: None => "" , Self :: Borrow => "&" , Self :: BorrowMut => "&mut " , Self :: Reborrow => "&*" , Self :: ReborrowMut => "&mut *" , } } }
};
}
