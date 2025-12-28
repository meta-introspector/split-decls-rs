macro_rules! deps {
    () => {
        Command!();
        Linker!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl dyn Linker + '_ { pub (crate) fn take_cmd (& mut self) -> Command { mem :: replace (self . cmd () , Command :: new ("")) } }
    };
}

impl_113!();