macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! from_str {
    () => {
        deps!();
        pub fn from_str (s : & 'static str) -> Error { Error { kind : ErrorKind :: Other (s) , } }
    };
}

from_str!();