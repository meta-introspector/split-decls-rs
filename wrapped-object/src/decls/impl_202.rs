macro_rules! deps {
    () => {
        Result!();
        ImageFileHeader!();
        ImageSymbol!();
        CoffHeader!();
        ImageSymbolBytes!();
        ReadRef!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl CoffHeader for pe :: ImageFileHeader { type ImageSymbol = pe :: ImageSymbol ; type ImageSymbolBytes = pe :: ImageSymbolBytes ; fn is_type_bigobj () -> bool { false } fn machine (& self) -> u16 { self . machine . get (LE) } fn number_of_sections (& self) -> u32 { self . number_of_sections . get (LE) . into () } fn pointer_to_symbol_table (& self) -> u32 { self . pointer_to_symbol_table . get (LE) } fn number_of_symbols (& self) -> u32 { self . number_of_symbols . get (LE) } fn characteristics (& self) -> u16 { self . characteristics . get (LE) } fn parse < 'data , R : ReadRef < 'data > > (data : R , offset : & mut u64) -> read :: Result < & 'data Self > { let header = data . read :: < pe :: ImageFileHeader > (offset) . read_error ("Invalid COFF file header size or alignment") ? ; * offset = offset . checked_add (header . size_of_optional_header . get (LE) . into ()) . read_error ("Invalid COFF optional header size") ? ; Ok (header) } }
    };
}

impl_202!();