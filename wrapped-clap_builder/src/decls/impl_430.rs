macro_rules! deps {
    () => {
        ArgMatcher!();
        ArgMatches!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl Deref for ArgMatcher { type Target = ArgMatches ; fn deref (& self) -> & Self :: Target { & self . matches } }
    };
}

impl_430!();