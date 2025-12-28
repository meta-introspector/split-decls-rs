macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Waker {
    () => {
        deps!();
        # [doc = " A queue of threads blocked on channel operations."] # [doc = ""] # [doc = " This data structure is used by threads to register blocking operations and get woken up once"] # [doc = " an operation becomes ready."] pub (crate) struct Waker { # [doc = " A list of select operations."] selectors : Vec < Entry > , # [doc = " A list of operations waiting to be ready."] observers : Vec < Entry > , }
    };
}

Waker!();