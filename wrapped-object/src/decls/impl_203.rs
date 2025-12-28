macro_rules! deps {
    () => {
        CoffHeader!();
        ImageSymbolEx!();
        AnonObjectHeaderBigobj!();
        Error!();
        Result!();
        ImageSymbolExBytes!();
        ImageSymbolBytes!();
        ReadRef!();
        ImageSymbol!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl CoffHeader for pe :: AnonObjectHeaderBigobj { type ImageSymbol = pe :: ImageSymbolEx ; type ImageSymbolBytes = pe :: ImageSymbolExBytes ; fn is_type_bigobj () -> bool { true } fn machine (& self) -> u16 { self . machine . get (LE) } fn number_of_sections (& self) -> u32 { self . number_of_sections . get (LE) } fn pointer_to_symbol_table (& self) -> u32 { self . pointer_to_symbol_table . get (LE) } fn number_of_symbols (& self) -> u32 { self . number_of_symbols . get (LE) } fn characteristics (& self) -> u16 { 0 } fn parse < 'data , R : ReadRef < 'data > > (data : R , offset : & mut u64) -> read :: Result < & 'data Self > { let header = data . read :: < pe :: AnonObjectHeaderBigobj > (offset) . read_error ("Invalid COFF bigobj file header size or alignment") ? ; if header . sig1 . get (LE) != pe :: IMAGE_FILE_MACHINE_UNKNOWN || header . sig2 . get (LE) != 0xffff || header . version . get (LE) < 2 || header . class_id != pe :: ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID { return Err (read :: Error ("Invalid COFF bigobj header values")) ; } Ok (header) } }
    };
}

impl_203!();