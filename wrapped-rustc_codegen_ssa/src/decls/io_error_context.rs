macro_rules! io_error_context {
    () => {
        fn io_error_context (context : & str , err : io :: Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , format ! ("{context}: {err}")) }
    };
}

io_error_context!();