macro_rules! deps {
    () => {
        ColorChoice!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl ColorChoice { # [doc = " Get the current [`ColorChoice`] state"] pub fn global () -> Self { USER . get () } # [doc = " Override the detected [`ColorChoice`]"] pub fn write_global (self) { USER . set (self) ; } }
    };
}

impl_1!();