macro_rules! ReadKind {
    () => {
        # [doc = " Kind of read access to a value"] # [doc = " (For informational purposes only)"] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ReadKind { Borrow (BorrowKind) , Copy , }
    };
}

ReadKind!();