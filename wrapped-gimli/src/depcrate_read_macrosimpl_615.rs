// Generated macro for impl_615 (impl)
macro_rules! Depcrate_read_macrosimpl_615 {
() => {
// Module: crate::read::macros
// Provides: {"impl_615"}
// Dependencies: {}
impl < R : Reader > DebugMacro < R > { # [doc = " Look up a macro reference the `.debug_macinfo` section by DebugMacroOffset."] # [doc = ""] # [doc = " A macinfo offset points to a list of macro information entries in the `.debug_macinfo` section."] # [doc = " To handle this, the function returns an iterator."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugMacro, DebugMacroOffset, LittleEndian};"] # [doc = ""] # [doc = " # fn main() -> Result<(), gimli::Error> {"] # [doc = " # let buf = [0x05, 0x00, 0x00, 0x01, 0x00, 0x5f, 0x5f, 0x53, 0x54, 0x44, 0x43, 0x5f, 0x5f, 0x20, 0x31, 0x00, 0x00];"] # [doc = " # let offset = DebugMacroOffset(0);"] # [doc = " # let read_section_somehow = || &buf;"] # [doc = " # let debug_macro_offset_somehow = || offset;"] # [doc = " let debug_macro = DebugMacro::new(read_section_somehow(), LittleEndian);"] # [doc = " let mut iter = debug_macro.get_macros(debug_macro_offset_somehow())?;"] # [doc = " while let Some(cur_macro) = iter.next()? {"] # [doc = "     println!(\"Found macro info {:?}\", cur_macro);"] # [doc = " }"] # [doc = " # Ok(()) }"] # [doc = " ```"] pub fn get_macros (& self , offset : DebugMacroOffset < R :: Offset >) -> Result < MacroIter < R > > { let mut input = self . section . clone () ; input . skip (offset . 0) ? ; let header = MacroUnitHeader :: parse (& mut input) ? ; Ok (MacroIter { input , format : header . format () , is_macro : true , }) } }
};
}
