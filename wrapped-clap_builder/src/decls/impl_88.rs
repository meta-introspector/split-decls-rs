macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl From < & '_ Command > for Command { fn from (cmd : & '_ Command) -> Self { cmd . clone () } }
    };
}

impl_88!()