// Generated macro for macro_296 (macro)
macro_rules! Depcrate_unistdmacro_296 {
() => {
// Module: crate::unistd
// Provides: {"macro_296"}
// Dependencies: {}
# [cfg (feature = "process")] # [cfg (target_os = "freebsd")] libc_bitflags ! { # [doc = " Flags for [`rfork`]"] # [doc = ""] # [doc = " subset of flags supported by FreeBSD 12.x and onwards"] # [doc = " with a safe outcome, thus as `RFMEM` can possibly lead to undefined behavior,"] # [doc = " it is not in the list. And `rfork_thread` is deprecated."] pub struct RforkFlags : libc :: c_int { # [doc = " creates a new process."] RFPROC ; # [doc = " the child process will detach from the parent."] # [doc = " however, no status will be emitted at child's exit."] RFNOWAIT ; # [doc = " the file descriptor's table will be copied"] RFFDG ; # [doc = " a new file descriptor's table will be created"] RFCFDG ; # [doc = " force sharing the sigacts structure between"] # [doc = " the child and the parent."] RFSIGSHARE ; # [doc = " enables kernel thread support."] RFTHREAD ; # [doc = " sets a status to emit at child's exit."] RFTSIGZMB ; # [doc = " linux's behavior compatibility setting."] # [doc = " emits SIGUSR1 as opposed to SIGCHLD upon child's exit."] RFLINUXTHPN ; } }
};
}
