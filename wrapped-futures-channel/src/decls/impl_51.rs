macro_rules! deps {
    () => {
        TrySendError!();
        SendError!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T > TrySendError < T > { # [doc = " Returns `true` if this error is a result of the channel being full."] pub fn is_full (& self) -> bool { self . err . is_full () } # [doc = " Returns `true` if this error is a result of the receiver being dropped."] pub fn is_disconnected (& self) -> bool { self . err . is_disconnected () } # [doc = " Returns the message that was attempted to be sent but failed."] pub fn into_inner (self) -> T { self . val } # [doc = " Drops the message and converts into a `SendError`."] pub fn into_send_error (self) -> SendError { self . err } }
    };
}

impl_51!()