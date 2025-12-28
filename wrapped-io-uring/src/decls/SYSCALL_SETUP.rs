macro_rules! SYSCALL_SETUP {
    () => {
        # [cfg (not (feature = "bindgen"))] const SYSCALL_SETUP : c_long = libc :: SYS_io_uring_setup ;
    };
}

SYSCALL_SETUP!()