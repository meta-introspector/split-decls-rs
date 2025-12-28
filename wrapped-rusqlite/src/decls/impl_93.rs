macro_rules! deps {
    () => {
        Connection!();
        ConnectionRef!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Deref for ConnectionRef < '_ > { type Target = Connection ; # [inline] fn deref (& self) -> & Connection { & self . conn } }
    };
}

impl_93!()