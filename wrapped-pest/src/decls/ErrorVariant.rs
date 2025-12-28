macro_rules! ErrorVariant {
    () => {
        # [doc = " Different kinds of parsing errors."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub enum ErrorVariant < R > { # [doc = " Generated parsing error with expected and unexpected `Rule`s"] ParsingError { # [doc = " Positive attempts"] positives : Vec < R > , # [doc = " Negative attempts"] negatives : Vec < R > , } , # [doc = " Custom error with a message"] CustomError { # [doc = " Short explanation"] message : String , } , }
    };
}

ErrorVariant!()