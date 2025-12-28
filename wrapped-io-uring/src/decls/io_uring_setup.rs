macro_rules! io_uring_setup {
    () => {
        # [cfg (feature = "direct-syscall")] pub unsafe fn io_uring_setup (entries : c_uint , p : * mut io_uring_params) -> io :: Result < c_int > { to_result (sc :: syscall2 (SYSCALL_SETUP as usize , entries as usize , p as usize) as _) }
    };
}

io_uring_setup!()