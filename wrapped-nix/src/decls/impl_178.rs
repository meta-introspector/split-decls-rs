macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl AsRef < timeval > for TimeVal { fn as_ref (& self) -> & timeval { & self . 0 } }
    };
}

impl_178!();