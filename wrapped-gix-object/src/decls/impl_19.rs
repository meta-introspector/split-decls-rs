macro_rules! deps {
    () => {
        BodyRef!();
        Trailers!();
        CommitRef!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'a > CommitRef < 'a > { # [doc = " Return exactly the same message as [`MessageRef::summary()`]."] pub fn message_summary (& self) -> Cow < 'a , BStr > { summary (self . message) } # [doc = " Return an iterator over message trailers as obtained from the last paragraph of the commit message."] # [doc = " Maybe empty."] pub fn message_trailers (& self) -> body :: Trailers < 'a > { BodyRef :: from_bytes (self . message) . trailers () } }
    };
}

impl_19!()