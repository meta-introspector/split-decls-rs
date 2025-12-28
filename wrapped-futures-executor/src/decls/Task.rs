macro_rules! deps {
    () => {
        WakeHandle!();
        ThreadPool!();
    };
}

macro_rules! Task {
    () => {
        deps!();
        # [doc = " A task responsible for polling a future to completion."] struct Task { future : FutureObj < 'static , () > , exec : ThreadPool , wake_handle : Arc < WakeHandle > , }
    };
}

Task!()