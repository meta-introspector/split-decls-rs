macro_rules! deps {
    () => {
        Error!();
        Register!();
        Result!();
    };
}

macro_rules! impl_681 {
    () => {
        deps!();
        impl Register { pub (crate) fn from_u64 (x : u64) -> Result < Register > { let y = x as u16 ; if u64 :: from (y) == x { Ok (Register (y)) } else { Err (Error :: UnsupportedRegister (x)) } } }
    };
}

impl_681!()