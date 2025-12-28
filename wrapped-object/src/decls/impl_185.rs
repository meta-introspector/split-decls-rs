macro_rules! deps {
    () => {
        Bytes!();
        U64Bytes!();
        SymbolIteratorInternal!();
        Result!();
        ArchiveKind!();
        ArchiveSymbolIterator!();
        ReadRef!();
        U16Bytes!();
        U32Bytes!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'data > ArchiveSymbolIterator < 'data > { fn new < R : ReadRef < 'data > > (kind : ArchiveKind , data : R , offset : u64 , size : u64 ,) -> Result < Self , () > { let mut data = data . read_bytes_at (offset , size) . map (Bytes) ? ; match kind { ArchiveKind :: Unknown => Ok (ArchiveSymbolIterator (SymbolIteratorInternal :: None)) , ArchiveKind :: Gnu => { let offsets_count = data . read :: < U32Bytes < BE > > () ? . get (BE) ; let offsets = data . read_slice :: < U32Bytes < BE > > (offsets_count as usize) ? ; Ok (ArchiveSymbolIterator (SymbolIteratorInternal :: Gnu { offsets : offsets . iter () , names : data , })) } ArchiveKind :: Gnu64 => { let offsets_count = data . read :: < U64Bytes < BE > > () ? . get (BE) ; let offsets = data . read_slice :: < U64Bytes < BE > > (offsets_count as usize) ? ; Ok (ArchiveSymbolIterator (SymbolIteratorInternal :: Gnu64 { offsets : offsets . iter () , names : data , })) } ArchiveKind :: Bsd => { let offsets_size = data . read :: < U32Bytes < LE > > () ? . get (LE) ; let offsets = data . read_slice :: < [U32Bytes < LE > ; 2] > (offsets_size as usize / 8) ? ; let names_size = data . read :: < U32Bytes < LE > > () ? . get (LE) ; let names = data . read_bytes (names_size as usize) ? ; Ok (ArchiveSymbolIterator (SymbolIteratorInternal :: Bsd { offsets : offsets . iter () , names , })) } ArchiveKind :: Bsd64 => { let offsets_size = data . read :: < U64Bytes < LE > > () ? . get (LE) ; let offsets = data . read_slice :: < [U64Bytes < LE > ; 2] > (offsets_size as usize / 16) ? ; let names_size = data . read :: < U64Bytes < LE > > () ? . get (LE) ; let names = data . read_bytes (names_size as usize) ? ; Ok (ArchiveSymbolIterator (SymbolIteratorInternal :: Bsd64 { offsets : offsets . iter () , names , })) } ArchiveKind :: Coff => { let members_count = data . read :: < U32Bytes < LE > > () ? . get (LE) ; let members = data . read_slice :: < U32Bytes < LE > > (members_count as usize) ? ; let indices_count = data . read :: < U32Bytes < LE > > () ? . get (LE) ; let indices = data . read_slice :: < U16Bytes < LE > > (indices_count as usize) ? ; Ok (ArchiveSymbolIterator (SymbolIteratorInternal :: Coff { members , indices : indices . iter () , names : data , })) } ArchiveKind :: AixBig => Ok (ArchiveSymbolIterator (SymbolIteratorInternal :: None)) , } } }
    };
}

impl_185!();