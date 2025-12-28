macro_rules! deps {
    () => {
        PollNext!();
    };
}

macro_rules! impl_797 {
    () => {
        deps!();
        impl PollNext { # [doc = " Toggle the value and return the old one."] # [must_use] pub fn toggle (& mut self) -> Self { let old = * self ; * self = self . other () ; old } fn other (& self) -> Self { match self { Self :: Left => Self :: Right , Self :: Right => Self :: Left , } } }
    };
}

impl_797!();