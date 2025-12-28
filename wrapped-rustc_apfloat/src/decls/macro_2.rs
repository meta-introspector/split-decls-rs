macro_rules! macro_2 {
    () => {
        bitflags ! { # [doc = " IEEE-754R 7: Default exception handling."] # [doc = ""] # [doc = " UNDERFLOW or OVERFLOW are always returned or-ed with INEXACT."] # [doc = ""] # [doc = " APFloat models this behavior specified by IEEE-754:"] # [doc = "   \"For operations producing results in floating-point format, the default"] # [doc = "    result of an operation that signals the invalid operation exception"] # [doc = "    shall be a quiet NaN.\""] # [must_use] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug)] pub struct Status : u8 { const OK = 0x00 ; const INVALID_OP = 0x01 ; const DIV_BY_ZERO = 0x02 ; const OVERFLOW = 0x04 ; const UNDERFLOW = 0x08 ; const INEXACT = 0x10 ; } }
    };
}

macro_2!()