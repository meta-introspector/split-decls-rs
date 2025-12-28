macro_rules! deps {
    () => {
        TryRecvError!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl TryRecvError { # [doc = " Returns `true` if the channel is empty but not closed."] pub fn is_empty (& self) -> bool { matches ! (self , TryRecvError :: Empty) } # [doc = " Returns `true` if the channel is empty and closed."] pub fn is_closed (& self) -> bool { matches ! (self , TryRecvError :: Closed) } }
    };
}

impl_47!();