macro_rules! deps {
    () => {
        Single!();
    };
}

macro_rules! impl_647 {
    () => {
        deps!();
        impl < T > Single < T > { # [doc = " Constructs new `Single` with the given value."] fn new (val : T) -> Self { Self (Some (val)) } # [doc = " Attempts to take inner item immediately. Will always succeed if the stream isn't terminated."] fn next_immediate (& mut self) -> Option < T > { self . 0 . take () } }
    };
}

impl_647!();