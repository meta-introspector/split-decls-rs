macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! OwnedMutexLockFuture {
    () => {
        deps!();
        # [doc = " A future which resolves when the target mutex has been successfully acquired, owned version."] pub struct OwnedMutexLockFuture < T : ? Sized > { mutex : Option < Arc < Mutex < T > > > , wait_key : usize , }
    };
}

OwnedMutexLockFuture!()