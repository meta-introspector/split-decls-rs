macro_rules! deps {
    () => {
        EndianVec!();
        Writer!();
        Result!();
        Endianity!();
        Error!();
    };
}

macro_rules! impl_688 {
    () => {
        deps!();
        impl < Endian > Writer for EndianVec < Endian > where Endian : Endianity , { type Endian = Endian ; # [inline] fn endian (& self) -> Self :: Endian { self . endian } # [inline] fn len (& self) -> usize { self . vec . len () } fn write (& mut self , bytes : & [u8]) -> Result < () > { self . vec . extend (bytes) ; Ok (()) } fn write_at (& mut self , offset : usize , bytes : & [u8]) -> Result < () > { if offset > self . vec . len () { return Err (Error :: OffsetOutOfBounds) ; } let to = & mut self . vec [offset ..] ; if bytes . len () > to . len () { return Err (Error :: LengthOutOfBounds) ; } let to = & mut to [.. bytes . len ()] ; to . copy_from_slice (bytes) ; Ok (()) } }
    };
}

impl_688!();