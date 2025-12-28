macro_rules! deps {
    () => {
        ArangeHeader!();
        ArangeHeaderIter!();
        Reader!();
        Result!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl < R : Reader > ArangeHeaderIter < R > { # [doc = " Advance the iterator to the next header."] pub fn next (& mut self) -> Result < Option < ArangeHeader < R > > > { if self . input . is_empty () { return Ok (None) ; } let len = self . input . len () ; match ArangeHeader :: parse (& mut self . input , self . offset) { Ok (header) => { self . offset . 0 += len - self . input . len () ; Ok (Some (header)) } Err (e) => { self . input . empty () ; Err (e) } } } }
    };
}

impl_366!()