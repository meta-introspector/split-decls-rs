macro_rules! RwLock {
    () => {
        # [doc = " Mock implementation of `std::sync::RwLock`"] # [derive (Debug)] pub struct RwLock < T > { object : rt :: RwLock , data : std :: sync :: RwLock < T > , }
    };
}

RwLock!();