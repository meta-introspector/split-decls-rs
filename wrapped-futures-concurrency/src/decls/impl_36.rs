macro_rules! deps {
    () => {
        PollArray!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < const N : usize > DerefMut for PollArray < N > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . state } }
    };
}

impl_36!()