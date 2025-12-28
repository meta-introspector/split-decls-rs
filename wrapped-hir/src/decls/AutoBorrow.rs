macro_rules! AutoBorrow {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum AutoBorrow { # [doc = " Converts from T to &T."] Ref (Mutability) , # [doc = " Converts from T to *T."] RawPtr (Mutability) , }
    };
}

AutoBorrow!();