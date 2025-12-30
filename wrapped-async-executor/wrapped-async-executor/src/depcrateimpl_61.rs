// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl Drop for Runner < '_ > { fn drop (& mut self) { self . state . local_queues . write () . unwrap_or_else (PoisonError :: into_inner) . retain (| local | ! Arc :: ptr_eq (local , & self . local)) ; while let Ok (r) = self . local . pop () { r . schedule () ; } } }
};
}
