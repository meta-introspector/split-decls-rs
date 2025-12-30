// Generated macro for print_exports_trie (function)
macro_rules! Depcrate_readobj_machoprint_exports_trie {
() => {
// Module: crate::readobj::macho
// Provides: {"print_exports_trie"}
// Dependencies: {}
fn print_exports_trie < Mach : MachHeader > (p : & mut Printer < '_ > , endian : Mach :: Endian , linkedit : & LinkeditDataCommand < Mach :: Endian > , state : & MachState ,) { let Some (exports_trie) = linkedit . exports_trie (endian , state . linkedit_data) . print_err (p) else { return ; } ; for export_info in exports_trie { export_info . print_err (p) . map (| export_symbol | { p . group ("ExportSymbol" , | p | { p . field_inline_string ("Name" , export_symbol . name ()) ; p . field_hex ("Flags" , export_symbol . flags ()) ; p . flags (export_symbol . flags () , 0 , FLAGS_EXPORT_SYMBOL) ; p . flags (export_symbol . flags () , EXPORT_SYMBOL_FLAGS_KIND_MASK , FLAGS_EXPORT_SYMBOL_KIND ,) ; match export_symbol . data () { ExportData :: Regular { address } => p . field_hex ("Address" , address) , ExportData :: Reexport { dylib_ordinal , import_name , } => { p . field_hex ("DylibOrdinal" , dylib_ordinal) ; p . field_inline_string ("ImportName" , import_name) ; } ExportData :: StubAndResolver { stub_address , resolver_address , } => { p . field_hex ("StubAddress" , stub_address) ; p . field_hex ("ResolverAddress" , resolver_address) ; } } }) ; }) ; } }
};
}
