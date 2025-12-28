macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! MutexLockFuture {
    () => {
        deps!();
        # [doc = " A future which resolves when the target mutex has been successfully acquired."] pub struct MutexLockFuture < 'a , T : ? Sized > { mutex : Option < & 'a Mutex < T > > , wait_key : usize , }
    };
}

MutexLockFuture!();