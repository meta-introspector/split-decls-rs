// Generated macro for sealed (module)
macro_rules! Depcrate_randsealed {
() => {
// Module: crate::rand
// Provides: {"sealed"}
// Dependencies: {}
pub (crate) mod sealed { use crate :: error ; pub trait SecureRandom : core :: fmt :: Debug { # [doc = " Fills `dest` with random bytes."] fn fill_impl (& self , dest : & mut [u8]) -> Result < () , error :: Unspecified > ; } pub trait RandomlyConstructable : Sized { fn zero () -> Self ; fn as_mut_bytes (& mut self) -> & mut [u8] ; } impl < const T : usize > RandomlyConstructable for [u8 ; T] { # [inline] fn zero () -> Self { [0 ; T] } # [inline] fn as_mut_bytes (& mut self) -> & mut [u8] { & mut self [..] } } }
};
}
