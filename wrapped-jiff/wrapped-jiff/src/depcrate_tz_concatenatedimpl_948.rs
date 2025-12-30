// Generated macro for impl_948 (impl)
macro_rules! Depcrate_tz_concatenatedimpl_948 {
() => {
// Module: crate::tz::concatenated
// Provides: {"impl_948"}
// Dependencies: {}
# [cfg (test)] impl Read for [u8] { fn read_exact_at (& self , buf : & mut [u8] , offset : u64) -> Result < () , Error > { let offset = usize :: try_from (offset) . map_err (| _ | err ! ("offset `{offset}` overflowed `usize`")) ? ; let Some (slice) = self . get (offset ..) else { return Err (err ! ("given offset `{offset}` is not valid \
                 (only {len} bytes are available)" , len = self . len () ,)) ; } ; if buf . len () > slice . len () { return Err (err ! ("unexpected EOF, expected {len} bytes but only have {have}" , len = buf . len () , have = slice . len ())) ; } buf . copy_from_slice (& slice [.. buf . len ()]) ; Ok (()) } }
};
}
