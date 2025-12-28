macro_rules! deps {
    () => {
        Delay!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Delay { # [doc = " Creates a new future which will fire at `dur` time into the future."] # [inline] pub fn new (dur : Duration) -> Delay { Self (SendWrapper :: new (TimeoutFuture :: new (dur . as_millis () as u32))) } # [doc = " Resets the timeout."] # [inline] pub fn reset (& mut self , dur : Duration) { * self = Delay :: new (dur) ; } }
    };
}

impl_9!();