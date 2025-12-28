macro_rules! deps {
    () => {
        Result!();
        SymbolIteratorInternal!();
        ArchiveSymbolIterator!();
        ArchiveOffset!();
        Item!();
        ArchiveSymbol!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'data > Iterator for ArchiveSymbolIterator < 'data > { type Item = read :: Result < ArchiveSymbol < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . 0 { SymbolIteratorInternal :: None => None , SymbolIteratorInternal :: Gnu { offsets , names } => { let offset = offsets . next () ? . get (BE) ; Some (names . read_string () . read_error ("Missing archive symbol name") . map (| name | ArchiveSymbol { name , offset : ArchiveOffset (offset . into ()) , }) ,) } SymbolIteratorInternal :: Gnu64 { offsets , names } => { let offset = offsets . next () ? . get (BE) ; Some (names . read_string () . read_error ("Missing archive symbol name") . map (| name | ArchiveSymbol { name , offset : ArchiveOffset (offset) , }) ,) } SymbolIteratorInternal :: Bsd { offsets , names } => { let entry = offsets . next () ? ; Some (names . read_string_at (entry [0] . get (LE) as usize) . read_error ("Invalid archive symbol name offset") . map (| name | ArchiveSymbol { name , offset : ArchiveOffset (entry [1] . get (LE) . into ()) , }) ,) } SymbolIteratorInternal :: Bsd64 { offsets , names } => { let entry = offsets . next () ? ; Some (names . read_string_at (entry [0] . get (LE) as usize) . read_error ("Invalid archive symbol name offset") . map (| name | ArchiveSymbol { name , offset : ArchiveOffset (entry [1] . get (LE)) , }) ,) } SymbolIteratorInternal :: Coff { members , indices , names , } => { let index = indices . next () ? . get (LE) . wrapping_sub (1) ; let member = members . get (index as usize) . read_error ("Invalid archive symbol member index") ; let name = names . read_string () . read_error ("Missing archive symbol name") ; Some (member . and_then (| member | { name . map (| name | ArchiveSymbol { name , offset : ArchiveOffset (member . get (LE) . into ()) , }) })) } } } fn size_hint (& self) -> (usize , Option < usize >) { match & self . 0 { SymbolIteratorInternal :: None => (0 , None) , SymbolIteratorInternal :: Gnu { offsets , .. } => offsets . size_hint () , SymbolIteratorInternal :: Gnu64 { offsets , .. } => offsets . size_hint () , SymbolIteratorInternal :: Bsd { offsets , .. } => offsets . size_hint () , SymbolIteratorInternal :: Bsd64 { offsets , .. } => offsets . size_hint () , SymbolIteratorInternal :: Coff { indices , .. } => { indices . size_hint () } } } }
    };
}

impl_186!()