macro_rules! deps {
    () => {
        SendTimeoutError!();
        Timeout!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < T > SendTimeoutError < T > { # [doc = " Unwraps the message."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_channel::unbounded;"] # [doc = ""] # [doc = " let (s, r) = unbounded();"] # [doc = ""] # [doc = " if let Err(err) = s.send_timeout(\"foo\", Duration::from_secs(1)) {"] # [doc = "     assert_eq!(err.into_inner(), \"foo\");"] # [doc = " }"] # [doc = " ```"] pub fn into_inner (self) -> T { match self { Self :: Timeout (v) => v , Self :: Disconnected (v) => v , } } # [doc = " Returns `true` if the send operation timed out."] pub fn is_timeout (& self) -> bool { matches ! (self , Self :: Timeout (_)) } # [doc = " Returns `true` if the send operation failed because the channel is disconnected."] pub fn is_disconnected (& self) -> bool { matches ! (self , Self :: Disconnected (_)) } }
    };
}

impl_83!()