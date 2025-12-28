macro_rules! deps {
    () => {
        AddrEntryIter!();
        Result!();
        Reader!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < R : Reader > AddrEntryIter < R > { # [doc = " Advance the iterator and return the next address."] # [doc = ""] # [doc = " Returns the newly parsed address as `Ok(Some(addr))`. Returns `Ok(None)`"] # [doc = " when iteration is complete and all addresses have already been parsed and"] # [doc = " yielded. If an error occurs while parsing the next address, then this error"] # [doc = " is returned as `Err(e)`, and all subsequent calls return `Ok(None)`."] pub fn next (& mut self) -> Result < Option < u64 > > { if self . input . is_empty () { return Ok (None) ; } match self . input . read_address (self . encoding . address_size) { Ok (entry) => Ok (Some (entry)) , Err (e) => { self . input . empty () ; Err (e) } } } }
    };
}

impl_154!();