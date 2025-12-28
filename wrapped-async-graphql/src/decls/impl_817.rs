macro_rules! deps {
    () => {
        Upload!();
    };
}

macro_rules! impl_817 {
    () => {
        deps!();
        impl Deref for Upload { type Target = usize ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_817!()