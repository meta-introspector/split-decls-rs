macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! from_io {
    () => {
        deps!();
        pub fn from_io (e : io :: Error) -> Error { Error { kind : ErrorKind :: Io (e) , } }
    };
}

from_io!();