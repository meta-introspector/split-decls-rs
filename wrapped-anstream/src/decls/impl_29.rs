macro_rules! deps {
    () => {
        WinconBytesIter!();
        WinconBytes!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl WinconBytes { # [doc = " Initial state"] pub fn new () -> Self { Default :: default () } # [doc = " Strip the next segment of data"] pub fn extract_next < 's > (& 's mut self , bytes : & 's [u8]) -> WinconBytesIter < 's > { self . capture . reset () ; self . capture . printable . reserve (bytes . len ()) ; WinconBytesIter { bytes , parser : & mut self . parser , capture : & mut self . capture , } } }
    };
}

impl_29!();