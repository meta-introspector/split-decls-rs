macro_rules! deps {
    () => {
        EndianSlice!();
        Endianity!();
        Result!();
        Error!();
        DebugBytes!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < 'input , Endian : Endianity > fmt :: Debug for EndianSlice < 'input , Endian > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> core :: result :: Result < () , fmt :: Error > { fmt . debug_tuple ("EndianSlice") . field (& self . endian) . field (& DebugBytes (self . slice)) . finish () } }
    };
}

impl_288!();