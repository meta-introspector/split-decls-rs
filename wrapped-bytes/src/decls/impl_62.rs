macro_rules! deps {
    () => {
        BufMut!();
        Writer!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < B : BufMut + Sized > io :: Write for Writer < B > { fn write (& mut self , src : & [u8]) -> io :: Result < usize > { let n = cmp :: min (self . buf . remaining_mut () , src . len ()) ; self . buf . put_slice (& src [.. n]) ; Ok (n) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_62!()