macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! ParseIntegerError {
    () => {
        deps!();
        # [doc = " An error that can occur when parsing an integer."] # [doc = ""] # [doc = " * No digits"] # [doc = " * Invalid digit"] # [doc = " * Overflow"] # [doc = " * Underflow"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ParseIntegerError { kind : ErrorKind , }
    };
}

ParseIntegerError!();