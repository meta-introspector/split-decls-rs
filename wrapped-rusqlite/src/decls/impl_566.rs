macro_rules! deps {
    () => {
        ConnectionRef!();
        Connection!();
    };
}

macro_rules! impl_566 {
    () => {
        deps!();
        impl Deref for ConnectionRef < '_ > { type Target = Connection ; # [inline] fn deref (& self) -> & Connection { & self . conn } }
    };
}

impl_566!();