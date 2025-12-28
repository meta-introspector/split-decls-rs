macro_rules! Condvar {
    () => {
        # [doc = " Mock implementation of `std::sync::Condvar`."] # [derive (Debug)] pub struct Condvar { object : rt :: Condvar , }
    };
}

Condvar!();