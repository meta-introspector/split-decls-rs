// Generated macro for bake_kdf (function)
macro_rules! Depcratebake_kdf {
() => {
// Module: crate
// Provides: {"bake_kdf"}
// Dependencies: {}
# [doc = " `bake-kdf` key derivation algorithm described in STB 34.101.66-2014 8.1.4."] # [inline] pub fn bake_kdf (x : & [u8] , s : & [u8] , c : u128) -> [u8 ; 32] { let mut hasher = BeltHash :: default () ; hasher . update (x) ; hasher . update (s) ; let y = hasher . finalize_fixed () . 0 ; belt_keyrep (& y , & [0xFF ; 12] , & c . to_le_bytes ()) }
};
}
