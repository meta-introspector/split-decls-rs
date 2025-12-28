macro_rules! deps {
    () => {
        WaitTimeoutResult!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl WaitTimeoutResult { # [doc = " Returns whether the wait was known to have timed out."] # [inline] pub fn timed_out (self) -> bool { self . 0 } }
    };
}

impl_1!()