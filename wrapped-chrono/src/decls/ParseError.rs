macro_rules! deps {
    () => {
        ParseErrorKind!();
    };
}

macro_rules! ParseError {
    () => {
        deps!();
        # [doc = " An error from the `parse` function."] # [derive (Debug , Clone , PartialEq , Eq , Copy , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct ParseError (ParseErrorKind) ;
    };
}

ParseError!();