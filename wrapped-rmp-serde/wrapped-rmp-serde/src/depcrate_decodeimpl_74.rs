// Generated macro for impl_74 (impl)
macro_rules! Depcrate_decodeimpl_74 {
() => {
// Module: crate::decode
// Provides: {"impl_74"}
// Dependencies: {}
impl < R : Read , C > Deserializer < R , C > { # [inline] fn take_or_read_marker (& mut self) -> Result < Marker , MarkerReadError > { self . marker . take () . map_or_else (| | rmp :: decode :: read_marker (& mut self . rd) , Ok) } # [inline] fn peek_or_read_marker (& mut self) -> Result < Marker , MarkerReadError > { if let Some (m) = self . marker { Ok (m) } else { let m = rmp :: decode :: read_marker (& mut self . rd) ? ; Ok (self . marker . insert (m) . to_owned ()) } } }
};
}
