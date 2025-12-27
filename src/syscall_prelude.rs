// Syscall prelude - commonly used syscall functionality

// pub use split_decls_syscall_macros::{
//     syscall, safe_fs_read, safe_fs_write, safe_exec,
//     syscall_read, syscall_read_file, syscall_write, syscall_exec
// };
pub use crate::syscall_traits::{
    SyscallOracle, FileSystemOracle, ProcessOracle, NetworkOracle,
    DefaultSyscallOracle, DefaultFileSystemOracle, DefaultProcessOracle, DefaultNetworkOracle
};
pub use crate::syscall_oracle::{
    SyscallInterceptor, SyscallWrapper, SyscallAstTransformer, 
    OracleType, create_default_syscall_interceptor
};
