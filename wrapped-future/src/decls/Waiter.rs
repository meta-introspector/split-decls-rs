macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! Waiter {
    () => {
        deps!();
        pub struct Waiter (HANDLE) ;
    };
}

Waiter!();