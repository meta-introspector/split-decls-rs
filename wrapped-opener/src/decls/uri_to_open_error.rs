macro_rules! deps {
    () => {
        FilePathToUriError!();
        OpenError!();
    };
}

macro_rules! uri_to_open_error {
    () => {
        deps!();
        fn uri_to_open_error () -> OpenError { OpenError :: Io (io :: Error :: new (io :: ErrorKind :: InvalidInput , FilePathToUriError ,)) }
    };
}

uri_to_open_error!();