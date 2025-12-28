macro_rules! deps {
    () => {
        AddrHeader!();
        AddrHeaderIter!();
        Reader!();
        Result!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < R : Reader > AddrHeaderIter < R > { # [doc = " Advance the iterator to the next header."] pub fn next (& mut self) -> Result < Option < AddrHeader < R > > > { if self . input . is_empty () { return Ok (None) ; } let len = self . input . len () ; match AddrHeader :: parse (& mut self . input , self . offset) { Ok (header) => { self . offset . 0 += len - self . input . len () ; Ok (Some (header)) } Err (e) => { self . input . empty () ; Err (e) } } } }
    };
}

impl_149!();