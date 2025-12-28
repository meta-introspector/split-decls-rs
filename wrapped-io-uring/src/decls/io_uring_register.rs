macro_rules! io_uring_register {
    () => {
        # [cfg (feature = "direct-syscall")] pub unsafe fn io_uring_register (fd : c_int , opcode : c_uint , arg : * const c_void , nr_args : c_uint ,) -> io :: Result < c_int > { to_result (sc :: syscall4 (SYSCALL_REGISTER as usize , fd as usize , opcode as usize , arg as usize , nr_args as usize ,) as _) }
    };
}

io_uring_register!();