macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! from_exit {
    () => {
        deps!();
        pub fn from_exit (status : process :: ExitStatus) -> Error { Error { kind : ErrorKind :: Process (status) , } }
    };
}

from_exit!();