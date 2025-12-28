macro_rules! deps {
    () => {
        SendError!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T > SendError < T > { # [doc = " Unwraps the message."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_channel::unbounded;"] # [doc = ""] # [doc = " let (s, r) = unbounded();"] # [doc = " drop(r);"] # [doc = ""] # [doc = " if let Err(err) = s.send(\"foo\") {"] # [doc = "     assert_eq!(err.into_inner(), \"foo\");"] # [doc = " }"] # [doc = " ```"] pub fn into_inner (self) -> T { self . 0 } }
    };
}

impl_73!();