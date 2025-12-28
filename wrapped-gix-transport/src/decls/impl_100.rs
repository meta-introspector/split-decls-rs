macro_rules! deps {
    () => {
        Http!();
        HeadersThenBody!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < H : Http , B : Read + Unpin > Read for HeadersThenBody < H , B > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { self . handle_headers () ? ; self . body . read (buf) } }
    };
}

impl_100!();