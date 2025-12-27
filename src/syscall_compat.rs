// Syscall auditing wrapper for old syntax

#[macro_export]
macro_rules! ignore_syscall {
    (#[syscall="read"] $expr:expr) => { 
        {
            eprintln!("SYSCALL_AUDIT: read operation - {}", stringify!($expr));
            $expr
        }
    };
    (#[syscall="write"] $expr:expr) => { 
        {
            eprintln!("SYSCALL_AUDIT: write operation - {}", stringify!($expr));
            $expr
        }
    };
    (#[syscall="exec"] $expr:expr) => { 
        {
            eprintln!("SYSCALL_AUDIT: exec operation - {}", stringify!($expr));
            $expr
        }
    };
    (#[syscall=$type:literal] $expr:expr) => { 
        {
            eprintln!("SYSCALL_AUDIT: {} operation - {}", $type, stringify!($expr));
            $expr
        }
    };
}

pub use ignore_syscall;
