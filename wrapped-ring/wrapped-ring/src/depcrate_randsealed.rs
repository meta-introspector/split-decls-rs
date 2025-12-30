// Generated macro for sealed (module)
macro_rules! Depcrate_randsealed {
() => {
// Module: crate::rand
// Provides: {"sealed"}
// Dependencies: {}
pub (crate) mod sealed { use crate :: error ; pub trait SecureRandom : core :: fmt :: Debug { # [doc = " Fills `dest` with random bytes."] fn fill_impl (& self , dest : & mut [u8] , _ : crate :: sealed :: Arg ,) -> Result < () , error :: Unspecified > ; } pub trait RandomlyConstructable : Sized { fn zero () -> Self ; fn as_mut_bytes (& mut self) -> & mut [u8] ; } impl < const N : usize > RandomlyConstructable for [u8 ; N] { # [inline] fn zero () -> Self { [0 ; N] } # [inline] fn as_mut_bytes (& mut self) -> & mut [u8] { & mut self [..] } } }
};
}
