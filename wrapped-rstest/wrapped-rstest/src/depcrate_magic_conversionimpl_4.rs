// Generated macro for impl_4 (impl)
macro_rules! Depcrate_magic_conversionimpl_4 {
() => {
// Module: crate::magic_conversion
// Provides: {"impl_4"}
// Dependencies: {}
impl < 'a , T > ViaParseDebug < 'a , T > for & & Magic < T > where T : std :: str :: FromStr , T :: Err : std :: fmt :: Debug , { fn magic_conversion (& self , input : & 'a str) -> T { T :: from_str (input) . unwrap () } }
};
}
