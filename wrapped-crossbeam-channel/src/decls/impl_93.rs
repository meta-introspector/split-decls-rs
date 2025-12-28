macro_rules! deps {
    () => {
        Timeout!();
        RecvTimeoutError!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl RecvTimeoutError { # [doc = " Returns `true` if the receive operation timed out."] pub fn is_timeout (& self) -> bool { matches ! (self , Self :: Timeout) } # [doc = " Returns `true` if the receive operation failed because the channel is disconnected."] pub fn is_disconnected (& self) -> bool { matches ! (self , Self :: Disconnected) } }
    };
}

impl_93!()