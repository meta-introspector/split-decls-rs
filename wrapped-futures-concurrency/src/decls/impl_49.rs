macro_rules! deps {
    () => {
        PollState!();
        PollVec!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Deref for PollVec { type Target = [PollState] ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_49!()