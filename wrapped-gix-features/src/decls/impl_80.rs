macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < T , P > io :: Write for Write < T , P > where T : io :: Write , P : Progress , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let written = self . inner . write (buf) ? ; self . progress . inc_by (written) ; Ok (written) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
    };
}

impl_80!()