macro_rules! deps {
    () => {
        Duration!();
        ListFormat!();
    };
}

macro_rules! Mode {
    () => {
        deps!();
        # [derive (Debug , Clone)] # [doc = " Enum representing the execution mode."] pub (crate) enum Mode { # [doc = " Run benchmarks normally."] Benchmark , # [doc = " List all benchmarks but do not run them."] List (ListFormat) , # [doc = " Run benchmarks once to verify that they work, but otherwise do not measure them."] Test , # [doc = " Iterate benchmarks for a given length of time but do not analyze or report on them."] Profile (Duration) , }
    };
}

Mode!();