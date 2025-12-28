macro_rules! sleep_ms {
    () => {
        pub fn sleep_ms (ms : u64) { :: std :: thread :: sleep (Duration :: from_millis (ms)) ; }
    };
}

sleep_ms!();