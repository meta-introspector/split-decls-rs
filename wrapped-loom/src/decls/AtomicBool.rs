macro_rules! deps {
    () => {
        Atomic!();
    };
}

macro_rules! AtomicBool {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::sync::atomic::AtomicBool`."] # [doc = ""] # [doc = " NOTE: Unlike `std::sync::atomic::AtomicBool`, this type has a different"] # [doc = " in-memory representation than `bool`."] # [derive (Debug)] pub struct AtomicBool (Atomic < bool >) ;
    };
}

AtomicBool!()