macro_rules! deps {
    () => {
        Thread!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl Thread { fn explore (& mut self) { if * self == Thread :: Skip { * self = Thread :: Pending ; } } fn is_pending (& self) -> bool { * self == Thread :: Pending } fn is_active (& self) -> bool { * self == Thread :: Active } fn is_enabled (& self) -> bool { ! self . is_disabled () } fn is_disabled (& self) -> bool { * self == Thread :: Disabled } }
    };
}

impl_125!()