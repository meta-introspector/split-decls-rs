macro_rules! deps {
    () => {
        UnparkMutex!();
        Task!();
        ThreadPool!();
    };
}

macro_rules! WakeHandle {
    () => {
        deps!();
        struct WakeHandle { mutex : UnparkMutex < Task > , exec : ThreadPool , }
    };
}

WakeHandle!();