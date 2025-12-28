macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! CompareResult {
    () => {
        deps!();
        # [doc = " Indicates whether a comparison was successful, an error, or"] # [doc = " if more data was needed"] # [derive (Debug , Eq , PartialEq)] pub enum CompareResult { # [doc = " Comparison was successful"] Ok , # [doc = " We need more data to be sure"] Incomplete , # [doc = " Comparison failed"] Error , }
    };
}

CompareResult!()