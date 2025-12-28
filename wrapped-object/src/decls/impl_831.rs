macro_rules! deps {
    () => {
        FileAux!();
        FileAux32!();
    };
}

macro_rules! impl_831 {
    () => {
        deps!();
        impl FileAux for xcoff :: FileAux32 { fn x_fname (& self) -> & [u8 ; 8] { & self . x_fname } fn x_ftype (& self) -> u8 { self . x_ftype } fn x_auxtype (& self) -> Option < u8 > { None } }
    };
}

impl_831!()