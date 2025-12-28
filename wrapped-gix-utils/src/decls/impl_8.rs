macro_rules! deps {
    () => {
        Buffers!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Buffers { # [doc = " Clear all buffers, which should be called once processing is done."] pub fn clear (& mut self) { self . src . clear () ; self . dest . clear () ; } # [doc = " Must be called after every change (i.e. when it's known that `dest` was written."] pub fn swap (& mut self) { std :: mem :: swap (& mut self . src , & mut self . dest) ; } }
    };
}

impl_8!()