// Generated macro for impl_232 (impl)
macro_rules! Depcrateimpl_232 {
() => {
// Module: crate
// Provides: {"impl_232"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: io :: Write for Hasher { # [doc = " This is equivalent to [`update`](#method.update)."] # [inline] fn write (& mut self , input : & [u8]) -> std :: io :: Result < usize > { self . update (input) ; Ok (input . len ()) } # [inline] fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
