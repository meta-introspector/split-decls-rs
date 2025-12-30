// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl < W : Write > Write for BufWriterWithLineEndingFix < W > { # [cfg (windows)] fn write (& mut self , buf : & [u8]) -> Result < usize > { for & b in buf . iter () { if b == b'\n' && self . last_written != Some (b'\r') { self . inner . write (b"\r\n") } else { self . last_written = Some (b) ; self . inner . write (& [b]) } ? ; } Ok (buf . len ()) } # [cfg (not (windows))] # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize > { self . inner . write (buf) } # [inline] fn flush (& mut self) -> Result < () > { self . inner . flush () } }
};
}
