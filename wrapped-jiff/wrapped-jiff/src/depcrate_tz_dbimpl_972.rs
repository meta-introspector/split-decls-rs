// Generated macro for impl_972 (impl)
macro_rules! Depcrate_tz_dbimpl_972 {
() => {
// Module: crate::tz::db
// Provides: {"impl_972"}
// Dependencies: {}
impl core :: fmt :: Debug for TimeZoneDatabase { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "TimeZoneDatabase(") ? ; let Some (inner) = self . inner . as_deref () else { return write ! (f , "unavailable)") ; } ; match * inner { Kind :: ZoneInfo (ref db) => write ! (f , "{db:?}") ? , Kind :: Concatenated (ref db) => write ! (f , "{db:?}") ? , Kind :: Bundled (ref db) => write ! (f , "{db:?}") ? , } write ! (f , ")") } }
};
}
