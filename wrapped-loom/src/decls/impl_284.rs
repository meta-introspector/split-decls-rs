macro_rules! deps {
    () => {
        WaitTimeoutResult!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl WaitTimeoutResult { # [doc = " Returns `true` if the wait was known to have timed out."] pub fn timed_out (& self) -> bool { self . 0 } }
    };
}

impl_284!();