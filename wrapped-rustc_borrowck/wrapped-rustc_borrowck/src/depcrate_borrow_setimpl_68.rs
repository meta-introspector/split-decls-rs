// Generated macro for impl_68 (impl)
macro_rules! Depcrate_borrow_setimpl_68 {
() => {
// Module: crate::borrow_set
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'tcx > fmt :: Display for BorrowData < 'tcx > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let kind = match self . kind { mir :: BorrowKind :: Shared => "" , mir :: BorrowKind :: Fake (mir :: FakeBorrowKind :: Deep) => "fake " , mir :: BorrowKind :: Fake (mir :: FakeBorrowKind :: Shallow) => "fake shallow " , mir :: BorrowKind :: Mut { kind : mir :: MutBorrowKind :: ClosureCapture } => "uniq " , mir :: BorrowKind :: Mut { kind : mir :: MutBorrowKind :: Default | mir :: MutBorrowKind :: TwoPhaseBorrow , } => "mut " , } ; write ! (w , "&{:?} {}{:?}" , self . region , kind , self . borrowed_place) } }
};
}
