macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! impl_570 {
    () => {
        deps!();
        impl < 'b , 'c , T > Deref for Reference < 'b , 'c , T > where T : ? Sized + 'static , { type Target = T ; fn deref (& self) -> & Self :: Target { match * self { Reference :: Borrowed (b) => b , Reference :: Copied (c) => c , } } }
    };
}

impl_570!();