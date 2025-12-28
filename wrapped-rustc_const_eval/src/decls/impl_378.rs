macro_rules! deps {
    () => {
        TypeNamePrinter!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl Write for TypeNamePrinter < '_ > { fn write_str (& mut self , s : & str) -> std :: fmt :: Result { self . path . push_str (s) ; Ok (()) } }
    };
}

impl_378!()