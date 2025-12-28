macro_rules! deps {
    () => {
        Error!();
        Describe!();
        Buf!();
        DescribeFormatOptions!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < 'repo > Describe < 'repo > { # [doc = " Prints this describe result, returning the result as a string."] pub fn format (& self , opts : Option < & DescribeFormatOptions >) -> Result < String , Error > { let buf = Buf :: new () ; let opts = opts . map (| o | & o . raw as * const _) . unwrap_or (ptr :: null ()) ; unsafe { try_call ! (raw :: git_describe_format (buf . raw () , self . raw , opts)) ; } Ok (String :: from_utf8 (buf . to_vec ()) . unwrap ()) } }
    };
}

impl_290!();