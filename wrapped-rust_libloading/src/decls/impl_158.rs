macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T > ops :: Deref for Symbol < '_ , T > { type Target = T ; fn deref (& self) -> & T { ops :: Deref :: deref (& self . inner) } }
    };
}

impl_158!()