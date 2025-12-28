macro_rules! deps {
    () => {
        WeakRoot!();
        Root!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl crate :: WeakRoot for Weak < Root > { type Root = Arc < Root > ; fn upgrade (& self) -> Option < Self :: Root > { Weak :: upgrade (self) } }
    };
}

impl_18!();