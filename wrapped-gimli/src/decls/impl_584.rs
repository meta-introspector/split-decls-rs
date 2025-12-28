macro_rules! deps {
    () => {
        DebugStrOffset!();
        Result!();
        DebugStr!();
        Reader!();
        LittleEndian!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl < R : Reader > DebugStr < R > { # [doc = " Lookup a string from the `.debug_str` section by DebugStrOffset."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugStr, DebugStrOffset, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x01, 0x02, 0x00];"] # [doc = " # let offset = DebugStrOffset(0);"] # [doc = " # let read_debug_str_section_somehow = || &buf;"] # [doc = " # let debug_str_offset_somehow = || offset;"] # [doc = " let debug_str = DebugStr::new(read_debug_str_section_somehow(), LittleEndian);"] # [doc = " println!(\"Found string {:?}\", debug_str.get_str(debug_str_offset_somehow()));"] # [doc = " ```"] pub fn get_str (& self , offset : DebugStrOffset < R :: Offset >) -> Result < R > { let input = & mut self . debug_str_section . clone () ; input . skip (offset . 0) ? ; input . read_null_terminated_slice () } }
    };
}

impl_584!();