// Generated macro for impl_43 (impl)
macro_rules! Depcrate_integerimpl_43 {
() => {
// Module: crate::integer
// Provides: {"impl_43"}
// Dependencies: {}
impl TryFrom < & BStr > for Integer { type Error = Error ; fn try_from (s : & BStr) -> Result < Self , Self :: Error > { let s = std :: str :: from_utf8 (s) . map_err (| err | int_err (s) . with_err (err)) ? ; if let Ok (value) = s . parse () { return Ok (Self { value , suffix : None }) ; } if s . len () <= 1 { return Err (int_err (s)) ; } let last_idx = s . len () - 1 ; if ! s . is_char_boundary (last_idx) { return Err (int_err (s)) ; } let (number , suffix) = s . split_at (s . len () - 1) ; if let (Ok (value) , Ok (suffix)) = (number . parse () , suffix . parse ()) { Ok (Self { value , suffix : Some (suffix) , }) } else { Err (int_err (s)) } } }
};
}
