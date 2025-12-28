macro_rules! deps {
    () => {
        Buf!();
        Reader!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < B : Buf + Sized > io :: Read for Reader < B > { fn read (& mut self , dst : & mut [u8]) -> io :: Result < usize > { let len = cmp :: min (self . buf . remaining () , dst . len ()) ; Buf :: copy_to_slice (& mut self . buf , & mut dst [0 .. len]) ; Ok (len) } }
    };
}

impl_41!()