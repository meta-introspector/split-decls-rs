// Generated macro for SyscallNo (enum)
macro_rules! Depcrate_syscallSyscallNo {
() => {
// Module: crate::syscall
// Provides: {"SyscallNo"}
// Dependencies: {}
pub (crate) enum SyscallNo { # [doc = " number of the system call `exit`"] Exit = 0 , # [doc = " number of the system call `write`"] Write = 1 , # [doc = " number of the system call `read`"] Read = 2 , # [doc = " number of the system call `usleep`"] Usleep = 3 , # [doc = " number of the system call `getpid`"] Getpid = 4 , # [doc = " number of the system call `yield`"] Yield = 5 , # [doc = " number of the system call `read_entropy`"] ReadEntropy = 6 , # [doc = " number of the system call `get_processor_count`"] GetProcessorCount = 7 , # [doc = " number of the system call `close`"] Close = 8 , # [doc = " number of the system call `futex_wait`"] FutexWait = 9 , # [doc = " number of the system call `futex_wake`"] FutexWake = 10 , # [doc = " number of the system call `open`"] Open = 11 , # [doc = " number of the system call `writev`"] Writev = 12 , # [doc = " number of the system call `readv`"] Readv = 13 , }
};
}
