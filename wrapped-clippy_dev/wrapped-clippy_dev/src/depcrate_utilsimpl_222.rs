// Generated macro for impl_222 (impl)
macro_rules! Depcrate_utilsimpl_222 {
() => {
// Module: crate::utils
// Provides: {"impl_222"}
// Dependencies: {}
impl Version { # [doc = " Displays the version as a rust version. i.e. `x.y.0`"] # [must_use] pub fn rust_display (self) -> impl Display { struct X (Version) ; impl Display for X { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}.{}.0" , self . 0 . major , self . 0 . minor) } } X (self) } # [doc = " Displays the version as it should appear in clippy's toml files. i.e. `0.x.y`"] # [must_use] pub fn toml_display (self) -> impl Display { struct X (Version) ; impl Display for X { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "0.{}.{}" , self . 0 . major , self . 0 . minor) } } X (self) } }
};
}
