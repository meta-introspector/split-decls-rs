// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
impl TzifFixed < String , Abbreviation > { fn quote (& self) -> proc_macro2 :: TokenStream { let TzifFixed { ref name , version , checksum , ref designations , ref posix_tz , } = * self ; let name = name . as_ref () . unwrap () ; let posix_tz = posix_tz . as_ref () . map (| tz | { let tz = tz . quote () ; quote ! (Some (# tz)) }) . unwrap_or_else (| | quote ! (None)) ; quote ! { jiff :: shared :: TzifFixed { name : Some (# name) , version : # version , checksum : # checksum , designations : # designations , posix_tz : # posix_tz , } } } }
};
}
