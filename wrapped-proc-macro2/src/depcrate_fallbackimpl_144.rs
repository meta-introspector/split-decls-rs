// Generated macro for impl_144 (impl)
macro_rules! Depcrate_fallbackimpl_144 {
() => {
// Module: crate::fallback
// Provides: {"impl_144"}
// Dependencies: {}
# [cfg (all (span_locations , not (fuzzing)))] impl SourceMap { fn next_start_pos (& self) -> u32 { self . files . last () . unwrap () . span . hi + 1 } fn add_file (& mut self , src : & str) -> Span { let (len , lines) = lines_offsets (src) ; let lo = self . next_start_pos () ; let span = Span { lo , hi : lo + (len as u32) , } ; self . files . push (FileInfo { source_text : src . to_owned () , span , lines , char_index_to_byte_offset : BTreeMap :: new () , }) ; span } fn find (& self , span : Span) -> usize { match self . files . binary_search_by (| file | { if file . span . hi < span . lo { Ordering :: Less } else if file . span . lo > span . hi { Ordering :: Greater } else { assert ! (file . span_within (span)) ; Ordering :: Equal } }) { Ok (i) => i , Err (_) => unreachable ! ("Invalid span with no related FileInfo!") , } } fn filepath (& self , span : Span) -> String { let i = self . find (span) ; if i == 0 { "<unspecified>" . to_owned () } else { format ! ("<parsed string {}>" , i) } } fn fileinfo (& self , span : Span) -> & FileInfo { let i = self . find (span) ; & self . files [i] } fn fileinfo_mut (& mut self , span : Span) -> & mut FileInfo { let i = self . find (span) ; & mut self . files [i] } }
};
}
