macro_rules! SYSCALL_REGISTER {
    () => {
        # [cfg (not (feature = "bindgen"))] const SYSCALL_REGISTER : c_long = libc :: SYS_io_uring_register ;
    };
}

SYSCALL_REGISTER!()