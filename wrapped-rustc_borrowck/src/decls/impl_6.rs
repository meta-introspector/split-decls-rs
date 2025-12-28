macro_rules! deps {
    () => {
        BorrowData!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Display for BorrowData < 'tcx > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let kind = match self . kind { mir :: BorrowKind :: Shared => "" , mir :: BorrowKind :: Fake (mir :: FakeBorrowKind :: Deep) => "fake " , mir :: BorrowKind :: Fake (mir :: FakeBorrowKind :: Shallow) => "fake shallow " , mir :: BorrowKind :: Mut { kind : mir :: MutBorrowKind :: ClosureCapture } => "uniq " , mir :: BorrowKind :: Mut { kind : mir :: MutBorrowKind :: Default | mir :: MutBorrowKind :: TwoPhaseBorrow , } => "mut " , } ; write ! (w , "&{:?} {}{:?}" , self . region , kind , self . borrowed_place) } }
    };
}

impl_6!();