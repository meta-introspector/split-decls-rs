macro_rules! deps {
    () => {
        Task!();
        UnparkMutex!();
        ThreadPool!();
    };
}

macro_rules! WakeHandle {
    () => {
        deps!();
        struct WakeHandle { mutex : UnparkMutex < Task > , exec : ThreadPool , }
    };
}

WakeHandle!()