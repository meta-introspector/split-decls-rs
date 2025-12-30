// Generated macro for impl_634 (impl)
macro_rules! Depcrate_read_elf_versionimpl_634 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_634"}
// Dependencies: {}
impl < 'data , Elf : FileHeader > VernauxIterator < 'data , Elf > { pub (super) fn new (endian : Elf :: Endian , data : & 'data [u8] , count : u16) -> Self { VernauxIterator { endian , data : Bytes (data) , count , } } # [doc = " Return the next `Vernaux` entry."] pub fn next (& mut self) -> Result < Option < & 'data elf :: Vernaux < Elf :: Endian > > > { if self . count == 0 { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . count = 0 ; } else { self . count -= 1 ; } result } fn parse (& mut self) -> Result < & 'data elf :: Vernaux < Elf :: Endian > > { let vernaux = self . data . read_at :: < elf :: Vernaux < _ > > (0) . read_error ("ELF vernaux is too short") ? ; self . data . skip (vernaux . vna_next . get (self . endian) as usize) . read_error ("Invalid ELF vna_next") ? ; Ok (vernaux) } }
};
}
