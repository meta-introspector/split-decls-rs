macro_rules! deps {
    () => {
        Export!();
        Error!();
        ByteString!();
        Result!();
    };
}

macro_rules! impl_688 {
    () => {
        deps!();
        impl < 'a > Debug for Export < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: result :: Result < () , core :: fmt :: Error > { f . debug_struct ("Export") . field ("ordinal" , & self . ordinal) . field ("name" , & self . name . map (ByteString)) . field ("target" , & self . target) . finish () } }
    };
}

impl_688!()