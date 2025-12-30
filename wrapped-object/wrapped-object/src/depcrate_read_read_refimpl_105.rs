// Generated macro for impl_105 (impl)
macro_rules! Depcrate_read_read_refimpl_105 {
() => {
// Module: crate::read::read_ref
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a > ReadRef < 'a > for & 'a [u8] { fn len (self) -> Result < u64 > { self . len () . try_into () . map_err (| _ | ()) } fn read_bytes_at (self , offset : u64 , size : u64) -> Result < & 'a [u8] > { if size == 0 { return Ok (& []) ; } let offset : usize = offset . try_into () . map_err (| _ | ()) ? ; let size : usize = size . try_into () . map_err (| _ | ()) ? ; self . get (offset ..) . ok_or (()) ? . get (.. size) . ok_or (()) } fn read_bytes_at_until (self , range : Range < u64 > , delimiter : u8) -> Result < & 'a [u8] > { let start : usize = range . start . try_into () . map_err (| _ | ()) ? ; let end : usize = range . end . try_into () . map_err (| _ | ()) ? ; let bytes = self . get (start .. end) . ok_or (()) ? ; match memchr :: memchr (delimiter , bytes) { Some (len) => { bytes . get (.. len) . ok_or (()) } None => Err (()) , } } }
};
}
