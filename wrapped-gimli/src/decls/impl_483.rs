macro_rules! deps {
    () => {
        Format!();
        Reader!();
        MacroIter!();
        DebugMacinfo!();
        Result!();
        DebugMacinfoOffset!();
        LittleEndian!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < R : Reader > DebugMacinfo < R > { # [doc = " Look up a macro reference the `.debug_macinfo` section by DebugMacinfoOffset."] # [doc = ""] # [doc = " A macinfo offset points to a list of macro information entries in the `.debug_macinfo` section."] # [doc = " To handle this, the function returns an iterator."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugMacinfo, DebugMacinfoOffset, LittleEndian};"] # [doc = ""] # [doc = " # fn main() -> Result<(), gimli::Error> {"] # [doc = " # let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0, 0];"] # [doc = " # let offset = DebugMacinfoOffset(0);"] # [doc = " # let read_section_somehow = || &buf;"] # [doc = " # let debug_macinfo_offset_somehow = || offset;"] # [doc = " let debug_macinfo = DebugMacinfo::new(read_section_somehow(), LittleEndian);"] # [doc = " let mut iter = debug_macinfo.get_macinfo(debug_macinfo_offset_somehow())?;"] # [doc = " while let Some(macinfo) = iter.next()? {"] # [doc = "     println!(\"Found macro info {:?}\", macinfo);"] # [doc = " }"] # [doc = " # Ok(()) }"] # [doc = " ```"] pub fn get_macinfo (& self , offset : DebugMacinfoOffset < R :: Offset >) -> Result < MacroIter < R > > { let mut input = self . section . clone () ; input . skip (offset . 0) ? ; Ok (MacroIter { input , format : Format :: Dwarf32 , is_macro : false , }) } }
    };
}

impl_483!();