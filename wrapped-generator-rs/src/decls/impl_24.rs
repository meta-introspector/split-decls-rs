macro_rules! deps {
    () => {
        GeneratorImpl!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < A : Any , T : Any > GeneratorImpl < '_ , A , T > { # [doc = " create a new generator with default stack size"] fn init_context (& mut self) { unsafe { std :: ptr :: write (self . context . para . as_mut_ptr () , & mut self . para as & mut dyn Any ,) ; std :: ptr :: write (self . context . ret . as_mut_ptr () , & mut self . ret as & mut dyn Any) ; } } }
    };
}

impl_24!();