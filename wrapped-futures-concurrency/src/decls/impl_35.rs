macro_rules! deps {
    () => {
        PollArray!();
        PollState!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < const N : usize > Deref for PollArray < N > { type Target = [PollState] ; fn deref (& self) -> & Self :: Target { & self . state } }
    };
}

impl_35!();