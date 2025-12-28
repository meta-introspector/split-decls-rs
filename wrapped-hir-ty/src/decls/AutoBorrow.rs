macro_rules! deps {
    () => {
        AutoBorrowMutability!();
    };
}

macro_rules! AutoBorrow {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum AutoBorrow { # [doc = " Converts from T to &T."] Ref (AutoBorrowMutability) , # [doc = " Converts from T to *T."] RawPtr (Mutability) , }
    };
}

AutoBorrow!();