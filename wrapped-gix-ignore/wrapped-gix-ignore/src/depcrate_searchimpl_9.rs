// Generated macro for impl_9 (impl)
macro_rules! Depcrate_searchimpl_9 {
() => {
// Module: crate::search
// Provides: {"impl_9"}
// Dependencies: {}
impl Pattern for Ignore { type Value = crate :: Kind ; fn bytes_to_patterns (& self , bytes : & [u8] , _source : & std :: path :: Path) -> Vec < pattern :: Mapping < Self :: Value > > { crate :: parse (bytes , self . support_precious) . map (| (pattern , line_number , kind) | pattern :: Mapping { pattern , value : kind , sequence_number : line_number , }) . collect () } }
};
}
