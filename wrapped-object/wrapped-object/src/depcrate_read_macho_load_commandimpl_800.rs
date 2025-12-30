// Generated macro for impl_800 (impl)
macro_rules! Depcrate_read_macho_load_commandimpl_800 {
() => {
// Module: crate::read::macho::load_command
// Provides: {"impl_800"}
// Dependencies: {}
impl < E : Endian > macho :: SymtabCommand < E > { # [doc = " Return the symbol table that this command references."] pub fn symbols < 'data , Mach : MachHeader < Endian = E > , R : ReadRef < 'data > > (& self , endian : E , data : R ,) -> Result < SymbolTable < 'data , Mach , R > > { let symbols = data . read_slice_at (self . symoff . get (endian) . into () , self . nsyms . get (endian) as usize ,) . read_error ("Invalid Mach-O symbol table offset or size") ? ; let str_start : u64 = self . stroff . get (endian) . into () ; let str_end = str_start . checked_add (self . strsize . get (endian) . into ()) . read_error ("Invalid Mach-O string table length") ? ; let strings = StringTable :: new (data , str_start , str_end) ; Ok (SymbolTable :: new (symbols , strings)) } }
};
}
