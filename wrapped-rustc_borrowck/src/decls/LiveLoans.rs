macro_rules! LiveLoans {
    () => {
        pub (crate) type LiveLoans = SparseBitMatrix < PointIndex , BorrowIndex > ;
    };
}

LiveLoans!();