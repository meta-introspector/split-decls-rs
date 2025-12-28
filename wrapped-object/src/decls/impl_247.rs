macro_rules! deps {
    () => {
        ImageSymbol!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl ImageSymbol for pe :: ImageSymbol { fn raw_name (& self) -> & [u8 ; 8] { & self . name } fn value (& self) -> u32 { self . value . get (LE) } fn section_number (& self) -> i32 { let section_number = self . section_number . get (LE) ; if section_number >= pe :: IMAGE_SYM_SECTION_MAX { (section_number as i16) as i32 } else { section_number as i32 } } fn typ (& self) -> u16 { self . typ . get (LE) } fn storage_class (& self) -> u8 { self . storage_class } fn number_of_aux_symbols (& self) -> u8 { self . number_of_aux_symbols } }
    };
}

impl_247!()