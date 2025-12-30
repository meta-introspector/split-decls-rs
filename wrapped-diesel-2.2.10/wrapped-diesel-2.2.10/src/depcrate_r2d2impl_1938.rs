// Generated macro for impl_1938 (impl)
macro_rules! Depcrate_r2d2impl_1938 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1938"}
// Dependencies: {}
impl < T > ManageConnection for ConnectionManager < T > where T : R2D2Connection + Send + 'static , { type Connection = T ; type Error = Error ; fn connect (& self) -> Result < T , Error > { T :: establish (& self . database_url) . map_err (Error :: ConnectionError) } fn is_valid (& self , conn : & mut T) -> Result < () , Error > { conn . ping () . map_err (Error :: QueryError) } fn has_broken (& self , conn : & mut T) -> bool { std :: thread :: panicking () || conn . is_broken () } }
};
}
