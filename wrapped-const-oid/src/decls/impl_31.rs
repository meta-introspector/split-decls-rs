macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: ArcInvalid { arc } => write ! (f , "OID contains out-of-range arc: {arc}") , Error :: ArcTooBig => f . write_str ("OID contains arc which is larger than 32-bits") , Error :: Base128 => f . write_str ("OID contains arc with invalid base 128 encoding") , Error :: DigitExpected { actual } => { write ! (f , "expected digit, got '{}'" , char :: from (actual)) } Error :: Empty => f . write_str ("OID value is empty") , Error :: Length => f . write_str ("OID length invalid") , Error :: Overflow => f . write_str ("arithmetic calculation overflowed") , Error :: RepeatedDot => f . write_str ("repeated consecutive '..' characters in OID") , Error :: TrailingDot => f . write_str ("OID ends with invalid trailing '.'") , } } }
    };
}

impl_31!()