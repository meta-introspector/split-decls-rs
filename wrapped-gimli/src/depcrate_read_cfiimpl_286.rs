// Generated macro for impl_286 (impl)
macro_rules! Depcrate_read_cfiimpl_286 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'a , R : Reader > CallFrameInstructionIter < 'a , R > { # [doc = " Parse the next call frame instruction."] pub fn next (& mut self) -> Result < Option < CallFrameInstruction < R :: Offset > > > { if self . input . is_empty () { return Ok (None) ; } match CallFrameInstruction :: parse (& mut self . input , self . address_encoding , & self . parameters , self . vendor ,) { Ok (instruction) => Ok (Some (instruction)) , Err (e) => { self . input . empty () ; Err (e) } } } }
};
}
