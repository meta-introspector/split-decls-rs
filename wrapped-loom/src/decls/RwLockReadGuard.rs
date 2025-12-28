macro_rules! deps {
    () => {
        RwLock!();
    };
}

macro_rules! RwLockReadGuard {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::sync::RwLockReadGuard`"] # [derive (Debug)] pub struct RwLockReadGuard < 'a , T > { lock : & 'a RwLock < T > , data : Option < std :: sync :: RwLockReadGuard < 'a , T > > , }
    };
}

RwLockReadGuard!();