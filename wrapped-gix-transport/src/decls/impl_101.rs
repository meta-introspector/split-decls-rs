macro_rules! deps {
    () => {
        HeadersThenBody!();
        Http!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < H : Http , B : BufRead + Unpin > BufRead for HeadersThenBody < H , B > { fn fill_buf (& mut self) -> std :: io :: Result < & [u8] > { self . handle_headers () ? ; self . body . fill_buf () } fn consume (& mut self , amt : usize) { self . body . consume (amt) ; } }
    };
}

impl_101!()