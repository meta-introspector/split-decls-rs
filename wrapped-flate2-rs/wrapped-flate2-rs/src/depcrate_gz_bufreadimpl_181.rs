// Generated macro for impl_181 (impl)
macro_rules! Depcrate_gz_bufreadimpl_181 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_181"}
// Dependencies: {}
impl < R : BufRead > Read for GzEncoder < R > { fn read (& mut self , mut into : & mut [u8]) -> io :: Result < usize > { let mut amt = 0 ; if self . eof { return self . read_footer (into) ; } else if self . pos < self . header . len () { amt += copy (into , & self . header , & mut self . pos) ; if amt == into . len () { return Ok (amt) ; } let tmp = into ; into = & mut tmp [amt ..] ; } match self . inner . read (into) ? { 0 => { self . eof = true ; self . pos = 0 ; self . read_footer (into) } n => Ok (amt + n) , } } }
};
}
