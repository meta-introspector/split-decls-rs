macro_rules! io_err_is_dir {
    () => {
        fn io_err_is_dir (err : & std :: io :: Error) -> bool { let raw = err . raw_os_error () ; raw == Some (if cfg ! (windows) { 5 } else { 21 }) || raw == Some (20) }
    };
}

io_err_is_dir!()