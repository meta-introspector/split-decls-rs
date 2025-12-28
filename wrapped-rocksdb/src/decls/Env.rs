macro_rules! deps {
    () => {
        EnvWrapper!();
    };
}

macro_rules! Env {
    () => {
        deps!();
        # [doc = " An Env is an interface used by the rocksdb implementation to access"] # [doc = " operating system functionality like the filesystem etc. Callers"] # [doc = " may wish to provide a custom Env object when opening a database to"] # [doc = " get fine gain control; e.g., to rate limit file system operations."] # [doc = ""] # [doc = " All Env implementations are safe for concurrent access from"] # [doc = " multiple threads without any external synchronization."] # [doc = ""] # [doc = " Note: currently, C API behinds C++ API for various settings."] # [doc = " See also: `rocksdb/include/env.h`"] # [derive (Clone)] pub struct Env (pub (crate) Arc < EnvWrapper >) ;
    };
}

Env!()