macro_rules! deps {
    () => {
        OutputReader!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl digest :: XofReader for OutputReader { # [inline] fn read (& mut self , buffer : & mut [u8]) { self . fill (buffer) ; } }
    };
}

impl_133!();