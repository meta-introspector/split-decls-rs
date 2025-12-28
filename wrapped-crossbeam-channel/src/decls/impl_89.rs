macro_rules! deps {
    () => {
        TryRecvError!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl TryRecvError { # [doc = " Returns `true` if the receive operation failed because the channel is empty."] pub fn is_empty (& self) -> bool { matches ! (self , Self :: Empty) } # [doc = " Returns `true` if the receive operation failed because the channel is disconnected."] pub fn is_disconnected (& self) -> bool { matches ! (self , Self :: Disconnected) } }
    };
}

impl_89!()