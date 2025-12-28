macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Error { # [doc = " Escalate this error into a panic."] # [doc = ""] # [doc = " This is a workaround until `Result::unwrap` is allowed in `const fn`."] # [allow (clippy :: panic)] pub (crate) const fn panic (self) -> ! { match self { Error :: ArcInvalid { .. } | Error :: ArcTooBig => panic ! ("OID contains invalid arc") , Error :: Base128 => panic ! ("OID contains arc with invalid base 128 encoding") , Error :: DigitExpected { .. } => panic ! ("OID expected to start with digit") , Error :: Empty => panic ! ("OID value is empty") , Error :: Length => panic ! ("OID length invalid") , Error :: Overflow => panic ! ("arithmetic calculation overflowed") , Error :: RepeatedDot => panic ! ("repeated consecutive '..' characters in OID") , Error :: TrailingDot => panic ! ("OID ends with invalid trailing '.'") , } } }
    };
}

impl_30!()