// Generated macro for impl_71 (impl)
macro_rules! Depcrate_processimpl_71 {
() => {
// Module: crate::process
// Provides: {"impl_71"}
// Dependencies: {}
impl io :: Read for CommandReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let stdout = match self . child . stdout { None => return Ok (0) , Some (ref mut stdout) => stdout , } ; let nread = stdout . read (buf) ? ; if nread == 0 { self . eof = true ; self . close () . map (| _ | 0) } else { Ok (nread) } } }
};
}
