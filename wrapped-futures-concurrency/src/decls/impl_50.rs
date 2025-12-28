macro_rules! deps {
    () => {
        PollVec!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl DerefMut for PollVec { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_50!();