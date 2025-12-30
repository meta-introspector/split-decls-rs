// Generated macro for impl_311 (impl)
macro_rules! Depcrate_outputimpl_311 {
() => {
// Module: crate::output
// Provides: {"impl_311"}
// Dependencies: {}
impl Output for StringOutput { fn write (& mut self , seg : & str) -> Result < () , IOError > { self . buf . extend_from_slice (seg . as_bytes ()) ; Ok (()) } fn write_fmt (& mut self , args : std :: fmt :: Arguments < '_ >) -> Result < () , IOError > { self . buf . write_fmt (args) } }
};
}
