// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl Write for Host { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { unsafe { miri_write_to_stdout (s . as_bytes ()) ; } Ok (()) } }
};
}
