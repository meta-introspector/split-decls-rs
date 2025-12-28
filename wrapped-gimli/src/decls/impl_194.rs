macro_rules! deps {
    () => {
        Result!();
        CfiEntriesIter!();
        Reader!();
        Section!();
        CieOrFde!();
        UnwindSection!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'bases , Section , R > CfiEntriesIter < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { # [doc = " Advance the iterator to the next entry."] pub fn next (& mut self) -> Result < Option < CieOrFde < 'bases , Section , R > > > { loop { if self . input . is_empty () { return Ok (None) ; } match parse_cfi_entry (self . bases , & self . section , & mut self . input) { Ok (Some (entry)) => return Ok (Some (entry)) , Err (e) => { self . input . empty () ; return Err (e) ; } Ok (None) => { if Section :: has_zero_terminator () { self . input . empty () ; return Ok (None) ; } continue ; } } } } }
    };
}

impl_194!()