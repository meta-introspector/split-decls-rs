macro_rules! SYSCALL_ENTER {
    () => {
        # [cfg (not (feature = "bindgen"))] const SYSCALL_ENTER : c_long = libc :: SYS_io_uring_enter ;
    };
}

SYSCALL_ENTER!()