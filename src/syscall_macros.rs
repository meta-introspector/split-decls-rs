// Re-export syscall macros from separate proc-macro crate
// pub use split_decls_syscall_macros::*;

// Define stub macros to replace syscall functionality
#[macro_export]
macro_rules! ignore_syscall {
    ($($tt:tt)*) => {
        $($tt)*
    };
}

#[macro_export]
macro_rules! syscallexec {
    ($($tt:tt)*) => {
        $($tt)*
    };
}

// Cached file reading macro
#[macro_export]
macro_rules! cached_fs_read_to_string {
    ($path:expr) => {
        crate::syscall::cached_read_to_string($path)
    };
}
