macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " default error type, only contains the error's location and code"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Error < I > { # [doc = " position of the error in the input data"] pub input : I , # [doc = " nom error code"] pub code : ErrorKind , }
    };
}

Error!();