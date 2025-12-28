macro_rules! deps {
    () => {
        EndianVec!();
        Endianity!();
    };
}

macro_rules! impl_687 {
    () => {
        deps!();
        impl < Endian > EndianVec < Endian > where Endian : Endianity , { # [doc = " Construct an empty `EndianVec` with the given endianity."] pub fn new (endian : Endian) -> EndianVec < Endian > { EndianVec { vec : Vec :: new () , endian , } } # [doc = " Return a reference to the raw slice."] pub fn slice (& self) -> & [u8] { & self . vec } # [doc = " Convert into a `Vec<u8>`."] pub fn into_vec (self) -> Vec < u8 > { self . vec } # [doc = " Take any written data out of the `EndianVec`, leaving an empty `Vec` in its place."] pub fn take (& mut self) -> Vec < u8 > { let mut vec = Vec :: new () ; mem :: swap (& mut self . vec , & mut vec) ; vec } }
    };
}

impl_687!();