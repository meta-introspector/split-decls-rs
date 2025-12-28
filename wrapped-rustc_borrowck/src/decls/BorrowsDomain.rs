macro_rules! BorrowsDomain {
    () => {
        type BorrowsDomain = MixedBitSet < BorrowIndex > ;
    };
}

BorrowsDomain!();