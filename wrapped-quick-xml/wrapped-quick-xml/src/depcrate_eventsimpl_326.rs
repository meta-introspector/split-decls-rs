// Generated macro for impl_326 (impl)
macro_rules! Depcrate_eventsimpl_326 {
() => {
// Module: crate::events
// Provides: {"impl_326"}
// Dependencies: {}
impl < 'a > Event < 'a > { # [doc = " Converts the event to an owned version, untied to the lifetime of"] # [doc = " buffer used when reading but incurring a new, separate allocation."] pub fn into_owned (self) -> Event < 'static > { match self { Event :: Start (e) => Event :: Start (e . into_owned ()) , Event :: End (e) => Event :: End (e . into_owned ()) , Event :: Empty (e) => Event :: Empty (e . into_owned ()) , Event :: Text (e) => Event :: Text (e . into_owned ()) , Event :: Comment (e) => Event :: Comment (e . into_owned ()) , Event :: CData (e) => Event :: CData (e . into_owned ()) , Event :: Decl (e) => Event :: Decl (e . into_owned ()) , Event :: PI (e) => Event :: PI (e . into_owned ()) , Event :: DocType (e) => Event :: DocType (e . into_owned ()) , Event :: GeneralRef (e) => Event :: GeneralRef (e . into_owned ()) , Event :: Eof => Event :: Eof , } } # [doc = " Converts the event into a borrowed event."] # [inline] pub fn borrow (& self) -> Event < '_ > { match self { Event :: Start (e) => Event :: Start (e . borrow ()) , Event :: End (e) => Event :: End (e . borrow ()) , Event :: Empty (e) => Event :: Empty (e . borrow ()) , Event :: Text (e) => Event :: Text (e . borrow ()) , Event :: Comment (e) => Event :: Comment (e . borrow ()) , Event :: CData (e) => Event :: CData (e . borrow ()) , Event :: Decl (e) => Event :: Decl (e . borrow ()) , Event :: PI (e) => Event :: PI (e . borrow ()) , Event :: DocType (e) => Event :: DocType (e . borrow ()) , Event :: GeneralRef (e) => Event :: GeneralRef (e . borrow ()) , Event :: Eof => Event :: Eof , } } }
};
}
