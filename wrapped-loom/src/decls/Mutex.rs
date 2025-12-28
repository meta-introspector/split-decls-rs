macro_rules! Mutex {
    () => {
        # [doc = " Mock implementation of `std::sync::Mutex`."] # [derive (Debug)] pub struct Mutex < T : ? Sized > { object : rt :: Mutex , data : std :: sync :: Mutex < T > , }
    };
}

Mutex!();