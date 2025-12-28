macro_rules! deps {
    () => {
        FormatAsPaserk!();
        Id!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl FormatAsPaserk for Id { fn fmt (& self , write : & mut dyn Write) -> core :: fmt :: Result { write . write_str (& self . header) ? ; write . write_str (& self . identifier) } }
    };
}

impl_75!();