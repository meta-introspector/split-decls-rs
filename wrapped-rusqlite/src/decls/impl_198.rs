macro_rules! deps {
    () => {
        Sql!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl Deref for Sql { type Target = str ; fn deref (& self) -> & str { self . as_str () } }
    };
}

impl_198!()