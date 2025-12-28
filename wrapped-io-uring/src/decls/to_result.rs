macro_rules! to_result {
    () => {
        # [cfg (not (feature = "direct-syscall"))] fn to_result (ret : c_int) -> io :: Result < c_int > { if ret >= 0 { Ok (ret) } else { Err (io :: Error :: last_os_error ()) } }
    };
}

to_result!();