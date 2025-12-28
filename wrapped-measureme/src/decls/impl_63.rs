macro_rules! deps {
    () => {
        BackingStorage!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Write for BackingStorage { # [inline] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { match * self { BackingStorage :: File (ref mut file) => file . write (buf) , BackingStorage :: Memory (ref mut vec) => vec . write (buf) , } } fn flush (& mut self) -> std :: io :: Result < () > { match * self { BackingStorage :: File (ref mut file) => file . flush () , BackingStorage :: Memory (_) => { Ok (()) } } } }
    };
}

impl_63!()