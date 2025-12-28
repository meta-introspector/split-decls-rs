macro_rules! deps {
    () => {
        ByteRecords!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < B : io :: BufRead > Iterator for ByteRecords < B > { type Item = io :: Result < Vec < u8 > > ; fn next (& mut self) -> Option < io :: Result < Vec < u8 > > > { let mut bytes = vec ! [] ; match self . buf . read_until (self . terminator , & mut bytes) { Err (e) => Some (Err (e)) , Ok (0) => None , Ok (_) => { trim_record (& mut bytes , self . terminator) ; Some (Ok (bytes)) } } } }
    };
}

impl_150!()