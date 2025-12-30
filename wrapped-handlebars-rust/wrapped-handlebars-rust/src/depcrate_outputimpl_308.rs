// Generated macro for impl_308 (impl)
macro_rules! Depcrate_outputimpl_308 {
() => {
// Module: crate::output
// Provides: {"impl_308"}
// Dependencies: {}
impl < W : Write > Output for WriteOutput < W > { fn write (& mut self , seg : & str) -> Result < () , IOError > { self . write . write_all (seg . as_bytes ()) } fn write_fmt (& mut self , args : std :: fmt :: Arguments < '_ >) -> Result < () , IOError > { self . write . write_fmt (args) } }
};
}
