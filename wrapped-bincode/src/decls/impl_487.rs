macro_rules! deps {
    () => {
        Writer!();
        EncodeError!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < T : Writer > Writer for & mut T { # [inline] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { (* * self) . write (bytes) } }
    };
}

impl_487!()