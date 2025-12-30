// Generated macro for impl_1367 (impl)
macro_rules! Depcrate_utils_cacheimpl_1367 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1367"}
// Dependencies: {}
impl Interner { # [doc = " Interns a string reference, ensuring it is stored uniquely."] # [doc = ""] # [doc = " If the string has been previously interned, the same `Interned<String>` instance is returned."] pub fn intern_str (& self , s : & str) -> Interned < String > { self . strs . lock () . unwrap () . intern_borrow (s) } }
};
}
