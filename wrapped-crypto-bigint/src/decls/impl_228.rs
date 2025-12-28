macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < T : ? Sized > AsRef < T > for Odd < T > { fn as_ref (& self) -> & T { & self . 0 } }
    };
}

impl_228!()