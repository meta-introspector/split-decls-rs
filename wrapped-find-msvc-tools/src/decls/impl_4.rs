macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Deref for Env { type Target = OsStr ; fn deref (& self) -> & Self :: Target { match self { Env :: Owned (os_str) => os_str , Env :: Arced (os_str) => os_str , } } }
    };
}

impl_4!();