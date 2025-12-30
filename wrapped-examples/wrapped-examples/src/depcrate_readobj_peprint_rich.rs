// Generated macro for print_rich (function)
macro_rules! Depcrate_readobj_peprint_rich {
() => {
// Module: crate::readobj::pe
// Provides: {"print_rich"}
// Dependencies: {}
fn print_rich (p : & mut Printer < '_ > , data : & [u8] , offset : u64) { if ! p . options . pe_rich { return ; } if let Some (rich_header) = RichHeaderInfo :: parse (data , offset) { p . group ("RichHeader" , | p | { p . field_hex ("Offset" , rich_header . offset) ; p . field_hex ("Length" , rich_header . length) ; p . field_hex ("XorKey" , rich_header . xor_key) ; for entry in rich_header . unmasked_entries () { p . group ("RichHeaderEntry" , | p | { p . field ("ComponentId" , format ! ("0x{:08X}" , entry . comp_id)) ; p . field ("Count" , entry . count) ; }) ; } }) ; } }
};
}
