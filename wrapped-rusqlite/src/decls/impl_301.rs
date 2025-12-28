macro_rules! deps {
    () => {
        Savepoint!();
        Connection!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl Deref for Savepoint < '_ > { type Target = Connection ; # [inline] fn deref (& self) -> & Connection { self . conn } }
    };
}

impl_301!();