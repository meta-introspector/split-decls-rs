// Generated macro for warn_if_linked_with_gold (function)
macro_rules! Depcrate_back_linkwarn_if_linked_with_gold {
() => {
// Module: crate::back::link
// Provides: {"warn_if_linked_with_gold"}
// Dependencies: {}
fn warn_if_linked_with_gold (sess : & Session , path : & Path) -> Result < () , Box < dyn std :: error :: Error > > { use object :: read :: elf :: { FileHeader , SectionHeader } ; use object :: read :: { ReadCache , ReadRef , Result } ; use object :: { Endianness , elf } ; fn elf_has_gold_version_note < 'a > (elf : & impl FileHeader , data : impl ReadRef < 'a > ,) -> Result < bool > { let endian = elf . endian () ? ; let section = elf . sections (endian , data) ? . section_by_name (endian , b".note.gnu.gold-version") ; if let Some ((_ , section)) = section && let Some (mut notes) = section . notes (endian , data) ? { return Ok (notes . any (| note | { note . is_ok_and (| note | note . n_type (endian) == elf :: NT_GNU_GOLD_VERSION) })) ; } Ok (false) } let data = ReadCache :: new (BufReader :: new (File :: open (path) ?)) ; let was_linked_with_gold = if sess . target . pointer_width == 64 { let elf = elf :: FileHeader64 :: < Endianness > :: parse (& data) ? ; elf_has_gold_version_note (elf , & data) ? } else if sess . target . pointer_width == 32 { let elf = elf :: FileHeader32 :: < Endianness > :: parse (& data) ? ; elf_has_gold_version_note (elf , & data) ? } else { return Ok (()) ; } ; if was_linked_with_gold { let mut warn = sess . dcx () . struct_warn ("the gold linker is deprecated and has known bugs with Rust") ; warn . help ("consider using LLD or ld from GNU binutils instead") ; warn . emit () ; } Ok (()) }
};
}
