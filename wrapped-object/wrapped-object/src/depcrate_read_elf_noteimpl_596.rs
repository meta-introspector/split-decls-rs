// Generated macro for impl_596 (impl)
macro_rules! Depcrate_read_elf_noteimpl_596 {
() => {
// Module: crate::read::elf::note
// Provides: {"impl_596"}
// Dependencies: {}
impl < 'data , Endian : endian :: Endian > GnuPropertyIterator < 'data , Endian > { # [doc = " Returns the next property."] pub fn next (& mut self) -> read :: Result < Option < GnuProperty < 'data > > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> read :: Result < GnuProperty < 'data > > { (| | -> Result < _ , () > { let pr_type = self . data . read_at :: < U32 < Endian > > (0) ? . get (self . endian) ; let pr_datasz = self . data . read_at :: < U32 < Endian > > (4) ? . get (self . endian) as usize ; let pr_data = self . data . read_bytes_at (8 , pr_datasz) ? . 0 ; self . data . skip (util :: align (8 + pr_datasz , self . align)) ? ; Ok (GnuProperty { pr_type , pr_data }) }) () . read_error ("Invalid ELF GNU property") } }
};
}
