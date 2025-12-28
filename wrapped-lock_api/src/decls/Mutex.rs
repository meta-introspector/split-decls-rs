macro_rules! Mutex {
    () => {
        # [doc = " A mutual exclusion primitive useful for protecting shared data"] # [doc = ""] # [doc = " This mutex will block threads waiting for the lock to become available. The"] # [doc = " mutex can also be statically initialized or created via a `new`"] # [doc = " constructor. Each mutex has a type parameter which represents the data that"] # [doc = " it is protecting. The data can only be accessed through the RAII guards"] # [doc = " returned from `lock` and `try_lock`, which guarantees that the data is only"] # [doc = " ever accessed when the mutex is locked."] pub struct Mutex < R , T : ? Sized > { raw : R , data : UnsafeCell < T > , }
    };
}

Mutex!();