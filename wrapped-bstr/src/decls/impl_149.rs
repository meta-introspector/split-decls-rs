macro_rules! deps {
    () => {
        ByteLines!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < B : io :: BufRead > Iterator for ByteLines < B > { type Item = io :: Result < Vec < u8 > > ; fn next (& mut self) -> Option < io :: Result < Vec < u8 > > > { let mut bytes = vec ! [] ; match self . buf . read_until (b'\n' , & mut bytes) { Err (e) => Some (Err (e)) , Ok (0) => None , Ok (_) => { trim_line (& mut bytes) ; Some (Ok (bytes)) } } } }
    };
}

impl_149!()