// Generated macro for impl_440 (impl)
macro_rules! Depcrate_traitsimpl_440 {
() => {
// Module: crate::traits
// Provides: {"impl_440"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl ExtendInto for char { type Item = char ; type Extender = String ; # [inline] fn new_builder (& self) -> String { String :: new () } # [inline] fn extend_into (& self , acc : & mut String) { acc . push (* self) ; } }
};
}
