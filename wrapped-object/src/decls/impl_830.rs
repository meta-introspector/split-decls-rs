macro_rules! deps {
    () => {
        FileAux!();
        FileAux64!();
    };
}

macro_rules! impl_830 {
    () => {
        deps!();
        impl FileAux for xcoff :: FileAux64 { fn x_fname (& self) -> & [u8 ; 8] { & self . x_fname } fn x_ftype (& self) -> u8 { self . x_ftype } fn x_auxtype (& self) -> Option < u8 > { Some (self . x_auxtype) } }
    };
}

impl_830!()