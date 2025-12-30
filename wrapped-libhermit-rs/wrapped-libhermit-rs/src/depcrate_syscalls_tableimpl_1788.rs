// Generated macro for impl_1788 (impl)
macro_rules! Depcrate_syscalls_tableimpl_1788 {
() => {
// Module: crate::syscalls::table
// Provides: {"impl_1788"}
// Dependencies: {}
impl SyscallTable { pub const fn new () -> Self { let mut table = SyscallTable { handle : [sys_invalid as * const _ ; NO_SYSCALLS] , } ; table . handle [SYSNO_EXIT] = sys_exit as * const _ ; table . handle [SYSNO_WRITE] = sys_write as * const _ ; table . handle [SYSNO_READ] = sys_read as * const _ ; table . handle [SYSNO_USLEEP] = sys_usleep as * const _ ; table . handle [SYSNO_GETPID] = sys_getpid as * const _ ; table . handle [SYSNO_YIELD] = sys_yield as * const _ ; table . handle [SYSNO_READ_ENTROPY] = sys_read_entropy as * const _ ; table . handle [SYSNO_GET_PROCESSOR_COUNT] = sys_get_processor_count as * const _ ; table . handle [SYSNO_CLOSE] = sys_close as * const _ ; table . handle [SYSNO_FUTEX_WAIT] = sys_futex_wait as * const _ ; table . handle [SYSNO_FUTEX_WAKE] = sys_futex_wake as * const _ ; table . handle [SYSNO_OPEN] = sys_open as * const _ ; table . handle [SYSNO_READV] = sys_readv as * const _ ; table . handle [SYSNO_WRITEV] = sys_writev as * const _ ; table } }
};
}
