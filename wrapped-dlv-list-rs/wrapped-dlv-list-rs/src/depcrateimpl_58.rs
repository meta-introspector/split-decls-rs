// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > OccupiedEntry < T > { # [doc = " Convenience function for creating a new occupied entry."] # [must_use] pub fn new (generation : u64 , previous : Option < NonMaxUsize > , next : Option < NonMaxUsize > , value : T ,) -> OccupiedEntry < T > { OccupiedEntry { generation , next , previous , value , } } }
};
}
