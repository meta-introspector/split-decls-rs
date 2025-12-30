// Generated macro for print_dos (function)
macro_rules! Depcrate_readobj_peprint_dos {
() => {
// Module: crate::readobj::pe
// Provides: {"print_dos"}
// Dependencies: {}
fn print_dos (p : & mut Printer < '_ > , dos_header : & ImageDosHeader) { if ! p . options . file { return ; } p . group ("ImageDosHeader" , | p | { p . field_hex ("Magic" , dos_header . e_magic . get (LE)) ; p . field_hex ("CountBytesLastPage" , dos_header . e_cblp . get (LE)) ; p . field_hex ("CountPages" , dos_header . e_cp . get (LE)) ; p . field_hex ("CountRelocations" , dos_header . e_crlc . get (LE)) ; p . field_hex ("CountHeaderParagraphs" , dos_header . e_cparhdr . get (LE)) ; p . field_hex ("MinAllocParagraphs" , dos_header . e_minalloc . get (LE)) ; p . field_hex ("MaxAllocParagraphs" , dos_header . e_maxalloc . get (LE)) ; p . field_hex ("StackSegment" , dos_header . e_ss . get (LE)) ; p . field_hex ("StackPointer" , dos_header . e_sp . get (LE)) ; p . field_hex ("Checksum" , dos_header . e_csum . get (LE)) ; p . field_hex ("InstructionPointer" , dos_header . e_ip . get (LE)) ; p . field_hex ("CodeSegment" , dos_header . e_cs . get (LE)) ; p . field_hex ("AddressOfRelocations" , dos_header . e_lfarlc . get (LE)) ; p . field_hex ("OverlayNumber" , dos_header . e_ovno . get (LE)) ; p . field_hex ("OemId" , dos_header . e_oemid . get (LE)) ; p . field_hex ("OemInfo" , dos_header . e_oeminfo . get (LE)) ; p . field_hex ("AddressOfNewHeader" , dos_header . e_lfanew . get (LE)) ; }) ; }
};
}
