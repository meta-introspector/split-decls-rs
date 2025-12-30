// Generated macro for impl_438 (impl)
macro_rules! Depcrate_traitsimpl_438 {
() => {
// Module: crate::traits
// Provides: {"impl_438"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl ExtendInto for str { type Item = char ; type Extender = String ; # [inline] fn new_builder (& self) -> String { String :: new () } # [inline] fn extend_into (& self , acc : & mut String) { acc . push_str (self) ; } }
};
}
