macro_rules! deps {
    () => {
        Connection!();
        Transaction!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl Deref for Transaction < '_ > { type Target = Connection ; # [inline] fn deref (& self) -> & Connection { self . conn } }
    };
}

impl_298!();