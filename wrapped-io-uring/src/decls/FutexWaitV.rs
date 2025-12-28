macro_rules! FutexWaitV {
    () => {
        # [doc = " Wrapper around `futex_waitv` as used in [`futex_waitv` system"] # [doc = " call](https://www.kernel.org/doc/html/latest/userspace-api/futex2.html)."] # [derive (Default , Debug , Clone , Copy)] # [repr (transparent)] pub struct FutexWaitV (sys :: futex_waitv) ;
    };
}

FutexWaitV!()