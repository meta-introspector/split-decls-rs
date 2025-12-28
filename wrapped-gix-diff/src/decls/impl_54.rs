macro_rules! deps {
    () => {
        Recorder!();
        Location!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [doc = " Builder"] impl Recorder { # [doc = " Obtain a copy of the currently tracked, full path of the entry."] pub fn track_location (mut self , location : Option < Location >) -> Self { self . location = location ; self } }
    };
}

impl_54!();