macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! from_utf8 {
    () => {
        deps!();
        pub fn from_utf8 (e : str :: Utf8Error) -> Error { Error { kind : ErrorKind :: Utf8 (e) , } }
    };
}

from_utf8!();