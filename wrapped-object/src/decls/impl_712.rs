macro_rules! deps {
    () => {
        U16!();
        Bytes!();
        Error!();
        RelocationIterator!();
        Result!();
        ImageBaseRelocation!();
        RelocationBlockIterator!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        impl < 'data > RelocationBlockIterator < 'data > { # [doc = " Construct a new iterator from the data of the `.reloc` section."] pub fn new (data : & 'data [u8]) -> Self { RelocationBlockIterator { data : Bytes (data) } } # [doc = " Read the next relocation page."] pub fn next (& mut self) -> Result < Option < RelocationIterator < 'data > > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> Result < RelocationIterator < 'data > > { let header = self . data . read :: < pe :: ImageBaseRelocation > () . read_error ("Invalid PE reloc section size") ? ; let virtual_address = header . virtual_address . get (LE) ; let size = header . size_of_block . get (LE) ; if size <= 8 || size & 3 != 0 { return Err (Error ("Invalid PE reloc block size")) ; } let count = (size - 8) / 2 ; let relocs = self . data . read_slice :: < U16 < LE > > (count as usize) . read_error ("Invalid PE reloc block size") ? . iter () ; Ok (RelocationIterator { virtual_address , size , relocs , }) } }
    };
}

impl_712!();