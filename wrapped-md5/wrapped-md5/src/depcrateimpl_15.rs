// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (feature = "std")] impl core :: io :: Write for Context { # [inline] fn write (& mut self , data : & [u8]) -> core :: io :: Result < usize > { self . consume (data) ; Ok (data . len ()) } # [inline] fn flush (& mut self) -> core :: io :: Result < () > { Ok (()) } }
};
}
