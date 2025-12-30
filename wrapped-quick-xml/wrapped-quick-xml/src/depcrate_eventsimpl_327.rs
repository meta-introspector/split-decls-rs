// Generated macro for impl_327 (impl)
macro_rules! Depcrate_eventsimpl_327 {
() => {
// Module: crate::events
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'a > Deref for Event < 'a > { type Target = [u8] ; fn deref (& self) -> & [u8] { match * self { Event :: Start (ref e) | Event :: Empty (ref e) => e , Event :: End (ref e) => e , Event :: Text (ref e) => e , Event :: Decl (ref e) => e , Event :: PI (ref e) => e , Event :: CData (ref e) => e , Event :: Comment (ref e) => e , Event :: DocType (ref e) => e , Event :: GeneralRef (ref e) => e , Event :: Eof => & [] , } } }
};
}
