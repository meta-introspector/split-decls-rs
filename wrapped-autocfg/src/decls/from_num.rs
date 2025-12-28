macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! from_num {
    () => {
        deps!();
        pub fn from_num (e : num :: ParseIntError) -> Error { Error { kind : ErrorKind :: Num (e) , } }
    };
}

from_num!()