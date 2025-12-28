macro_rules! macro_204 {
    () => {
        # [cfg (all (target_os = "linux" , any (target_arch = "aarch64" , target_arch = "s390x" , target_arch = "x86" , target_arch = "x86_64")))] feature ! { #! [feature = "ucontext"] # [allow (missing_docs)] pub mod ucontext ; }
    };
}

macro_204!();