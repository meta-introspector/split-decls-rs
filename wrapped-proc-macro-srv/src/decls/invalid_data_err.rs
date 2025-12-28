macro_rules! invalid_data_err {
    () => {
        fn invalid_data_err (e : impl Into < Box < dyn std :: error :: Error + Send + Sync > >) -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , e) }
    };
}

invalid_data_err!();