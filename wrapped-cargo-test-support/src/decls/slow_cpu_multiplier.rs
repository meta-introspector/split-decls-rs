macro_rules! slow_cpu_multiplier {
    () => {
        # [doc = " A way for to increase the cut off for all the time based test."] # [doc = ""] # [doc = " Some CI setups are much slower then the equipment used by Cargo itself."] # [doc = " Architectures that do not have a modern processor, hardware emulation, etc."] pub fn slow_cpu_multiplier (main : u64) -> Duration { static SLOW_CPU_MULTIPLIER : OnceLock < u64 > = OnceLock :: new () ; let slow_cpu_multiplier = SLOW_CPU_MULTIPLIER . get_or_init (| | { env :: var ("CARGO_TEST_SLOW_CPU_MULTIPLIER") . ok () . and_then (| m | m . parse () . ok ()) . unwrap_or (1) }) ; Duration :: from_secs (slow_cpu_multiplier * main) }
    };
}

slow_cpu_multiplier!();