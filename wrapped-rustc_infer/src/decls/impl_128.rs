macro_rules! deps {
    () => {
        Constraint!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl Constraint < '_ > { pub fn involves_placeholders (& self) -> bool { self . sub . is_placeholder () || self . sup . is_placeholder () } }
    };
}

impl_128!();