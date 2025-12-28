macro_rules! deps {
    () => {
        Attrs!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl ops :: Deref for Attrs { type Target = [Attr] ; fn deref (& self) -> & [Attr] { & self . 0 } }
    };
}

impl_12!();