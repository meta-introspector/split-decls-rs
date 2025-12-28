macro_rules! ParseError {
    () => {
        # [doc = " Error during logger directive parsing process."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct ParseError { details : String , }
    };
}

ParseError!()