macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ErrorKind {
    () => {
        deps!();
        # [derive (Debug)] enum ErrorKind { Io (io :: Error) , Num (num :: ParseIntError) , Process (process :: ExitStatus) , Utf8 (str :: Utf8Error) , Other (& 'static str) , }
    };
}

ErrorKind!();