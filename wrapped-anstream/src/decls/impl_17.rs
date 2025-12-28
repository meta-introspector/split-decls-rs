macro_rules! deps {
    () => {
        StripBytesIter!();
        StripBytes!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl StripBytes { # [doc = " Initial state"] pub fn new () -> Self { Default :: default () } # [doc = " Strip the next segment of data"] pub fn strip_next < 's > (& 's mut self , bytes : & 's [u8]) -> StripBytesIter < 's > { StripBytesIter { bytes , state : & mut self . state , utf8parser : & mut self . utf8parser , } } }
    };
}

impl_17!();