macro_rules! deps {
    () => {
        TimesRange!();
    };
}

macro_rules! Times {
    () => {
        deps!();
        # [derive (Debug , Default)] # [doc (hidden)] pub struct Times { # [doc = " How many times has the expectation already been called?"] count : AtomicUsize , range : TimesRange }
    };
}

Times!()