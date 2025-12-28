macro_rules! deps {
    () => {
        SendError!();
        SendErrorKind!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl SendError { # [doc = " Returns `true` if this error is a result of the channel being full."] pub fn is_full (& self) -> bool { matches ! (self . kind , SendErrorKind :: Full) } # [doc = " Returns `true` if this error is a result of the receiver being dropped."] pub fn is_disconnected (& self) -> bool { matches ! (self . kind , SendErrorKind :: Disconnected) } }
    };
}

impl_46!()