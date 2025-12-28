macro_rules! deps {
    () => {
        StripStrIter!();
        StripStr!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl StripStr { # [doc = " Initial state"] pub fn new () -> Self { Default :: default () } # [doc = " Strip the next segment of data"] pub fn strip_next < 's > (& 's mut self , data : & 's str) -> StripStrIter < 's > { StripStrIter { bytes : data . as_bytes () , state : & mut self . state , } } }
    };
}

impl_6!()