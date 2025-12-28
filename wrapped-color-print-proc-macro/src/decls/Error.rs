macro_rules! deps {
    () => {
        ErrorDetail!();
        Input!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Replacement to [`nom::error::Error`]."] # [derive (Debug , PartialEq)] pub struct Error < 'a > { pub input : Input < 'a > , pub code : ErrorKind , pub detail : Option < ErrorDetail < 'a > > , }
    };
}

Error!()