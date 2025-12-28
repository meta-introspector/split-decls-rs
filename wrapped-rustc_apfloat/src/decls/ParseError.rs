macro_rules! ParseError {
    () => {
        # [doc = " An error which can occur when parsing a floating point number from a string."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub struct ParseError (pub & 'static str) ;
    };
}

ParseError!()