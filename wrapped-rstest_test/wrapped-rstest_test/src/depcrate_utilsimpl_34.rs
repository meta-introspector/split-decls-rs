// Generated macro for impl_34 (impl)
macro_rules! Depcrate_utilsimpl_34 {
() => {
// Module: crate::utils
// Provides: {"impl_34"}
// Dependencies: {}
impl < ST > CountMessageOccurrence for ST where ST : AsRef < str > , { fn count < S : AsRef < str > > (& self , message : S) -> usize { self . as_ref () . lines () . filter (| line | line . contains (message . as_ref ())) . count () } fn count_regex < S : AsRef < str > > (& self , regex : S) -> usize { let regex = regex :: Regex :: new (regex . as_ref ()) . unwrap () ; self . as_ref () . lines () . filter (| line | regex . is_match (line)) . count () } }
};
}
