macro_rules! Timer {
    () => {
        struct Timer { interval : Duration , delay : Delay , }
    };
}

Timer!();