// Generated macro for impl_6 (impl)
macro_rules! Depcrate_magic_conversionimpl_6 {
() => {
// Module: crate::magic_conversion
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a , T > ViaParse < 'a , T > for & Magic < T > where T : std :: str :: FromStr , { fn magic_conversion (& self , input : & 'a str) -> T { match T :: from_str (input) { Ok (v) => v , Err (_) => { panic ! ("Cannot parse '{}' to get {}" , input , std :: any :: type_name ::< T > ()) ; } } } }
};
}
