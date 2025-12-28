macro_rules! io_uring_enter {
    () => {
        # [cfg (feature = "direct-syscall")] pub unsafe fn io_uring_enter (fd : c_int , to_submit : c_uint , min_complete : c_uint , flags : c_uint , arg : * const libc :: c_void , size : usize ,) -> io :: Result < c_int > { to_result (sc :: syscall6 (SYSCALL_ENTER as usize , fd as usize , to_submit as usize , min_complete as usize , flags as usize , arg as usize , size ,) as _) }
    };
}

io_uring_enter!()