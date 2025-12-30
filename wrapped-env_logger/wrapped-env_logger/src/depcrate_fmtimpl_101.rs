// Generated macro for impl_101 (impl)
macro_rules! Depcrate_fmtimpl_101 {
() => {
// Module: crate::fmt
// Provides: {"impl_101"}
// Dependencies: {}
impl < F > RecordFormat for F where F : Fn (& mut Formatter , & Record < '_ >) -> io :: Result < () > , { fn format (& self , formatter : & mut Formatter , record : & Record < '_ >) -> io :: Result < () > { (self) (formatter , record) } }
};
}
