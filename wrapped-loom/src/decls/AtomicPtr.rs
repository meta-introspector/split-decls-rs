macro_rules! deps {
    () => {
        Atomic!();
    };
}

macro_rules! AtomicPtr {
    () => {
        deps!();
        # [doc = " Mock implementation of `std::sync::atomic::AtomicPtr`."] # [doc = ""] # [doc = " NOTE: Unlike `std::sync::atomic::AtomicPtr`, this type has a different"] # [doc = " in-memory representation than `*mut T`."] pub struct AtomicPtr < T > (Atomic < * mut T >) ;
    };
}

AtomicPtr!();