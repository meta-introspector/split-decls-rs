macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! MutexGuard {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::sync::MutexGuard`."] # [derive (Debug)] pub struct MutexGuard < 'a , T : ? Sized > { lock : & 'a Mutex < T > , data : Option < std :: sync :: MutexGuard < 'a , T > > , }
    };
}

MutexGuard!();