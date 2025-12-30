// Generated macro for impl_77 (impl)
macro_rules! Depcrate_consoleimpl_77 {
() => {
// Module: crate::console
// Provides: {"impl_77"}
// Dependencies: {}
impl Write for Console { # [doc = " Writes a buffer to the console."] # [doc = " The content is buffered until a newline is encountered or the internal buffer is full."] # [doc = " To force early output, use [`flush`](Self::flush)."] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { if SERIAL_BUFFER_SIZE - self . buffer . len () >= buf . len () { self . buffer . extend_from_slice (buf) . unwrap () ; if buf . contains (& b'\n') { self . flush () ? ; } } else { self . device . write_all (& self . buffer) ? ; self . buffer . clear () ; if buf . len () >= SERIAL_BUFFER_SIZE { self . device . write_all (buf) ? ; } else { self . buffer . extend_from_slice (buf) . unwrap () ; if buf . contains (& b'\n') { self . flush () ? ; } } } Ok (buf . len ()) } # [doc = " Immediately writes everything in the internal buffer to the output."] fn flush (& mut self) -> Result < () , Self :: Error > { if ! self . buffer . is_empty () { self . device . write_all (& self . buffer) ? ; self . buffer . clear () ; } Ok (()) } }
};
}
