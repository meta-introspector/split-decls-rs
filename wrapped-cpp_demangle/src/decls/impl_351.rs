macro_rules! deps {
    () => {
        DemangleWrite!();
        Result!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl < W : fmt :: Write > DemangleWrite for W { fn write_string (& mut self , s : & str) -> fmt :: Result { fmt :: Write :: write_str (self , s) } }
    };
}

impl_351!();