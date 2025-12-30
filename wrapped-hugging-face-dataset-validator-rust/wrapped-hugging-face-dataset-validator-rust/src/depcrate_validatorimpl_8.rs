// Generated macro for impl_8 (impl)
macro_rules! Depcrate_validatorimpl_8 {
() => {
// Module: crate::validator
// Provides: {"impl_8"}
// Dependencies: {}
impl ValidationResult { pub fn new () -> Self { Self :: default () } pub fn merge (& mut self , other : & ValidationResult) { self . viewer |= other . viewer ; self . preview |= other . preview ; self . search |= other . search ; self . filter |= other . filter ; self . statistics |= other . statistics ; } pub fn capability_count (& self) -> usize { [self . viewer , self . preview , self . search , self . filter , self . statistics] . iter () . filter (| & & x | x) . count () } }
};
}
