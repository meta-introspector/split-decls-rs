// Generated macro for impl_100 (impl)
macro_rules! Depcrate_formatimpl_100 {
() => {
// Module: crate::format
// Provides: {"impl_100"}
// Dependencies: {}
impl fmt :: Write for ByteBufWriter < '_ > { fn write_str (& mut self , s : & str) -> fmt :: Result { let dest = & mut self . buffer [self . cursor ..] ; if dest . len () < s . len () { return Err (fmt :: Error) ; } dest [.. s . len ()] . copy_from_slice (s . as_bytes ()) ; self . cursor += s . len () ; Ok (()) } }
};
}
