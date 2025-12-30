// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
impl TzifOwned { fn quote (& self) -> proc_macro2 :: TokenStream { let TzifOwned { ref fixed , ref types , ref transitions } = * self ; let fixed = fixed . quote () ; let types = types . iter () . map (TzifLocalTimeType :: quote) ; let transitions = transitions . quote () ; quote ! { { static TZ : jiff :: tz :: TimeZone = jiff :: tz :: TimeZone :: __internal_from_tzif (& jiff :: shared :: TzifStatic { fixed : # fixed , types : & [# (# types) ,*] , transitions : # transitions , } . into_jiff ()) ; unsafe { TZ . copy () } } } } }
};
}
