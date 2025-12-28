macro_rules! deps {
    () => {
        DecodedLength!();
        Result!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl fmt :: Debug for DecodedLength { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { DecodedLength :: CLOSE_DELIMITED => f . write_str ("CLOSE_DELIMITED") , DecodedLength :: CHUNKED => f . write_str ("CHUNKED") , DecodedLength (n) => f . debug_tuple ("DecodedLength") . field (& n) . finish () , } } }
    };
}

impl_36!()