macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl AsMut < timeval > for TimeVal { fn as_mut (& mut self) -> & mut timeval { & mut self . 0 } }
    };
}

impl_179!()