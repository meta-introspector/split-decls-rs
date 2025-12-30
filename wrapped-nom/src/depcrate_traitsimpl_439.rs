// Generated macro for impl_439 (impl)
macro_rules! Depcrate_traitsimpl_439 {
() => {
// Module: crate::traits
// Provides: {"impl_439"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl ExtendInto for & str { type Item = char ; type Extender = String ; # [inline] fn new_builder (& self) -> String { String :: new () } # [inline] fn extend_into (& self , acc : & mut String) { acc . push_str (self) ; } }
};
}
