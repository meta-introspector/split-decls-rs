// Generated macro for impl_120 (impl)
macro_rules! Depcrate_parseimpl_120 {
() => {
// Module: crate::parse
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'input , 'b , 's > FootnoteDefs < 'input > where 's : 'b , { # [doc = " Performs a lookup on reference label using unicode case folding."] pub fn contains (& 's self , key : & 'b str) -> bool { self . 0 . contains_key (& UniCase :: new (key . into ())) } # [doc = " Performs a lookup on reference label using unicode case folding."] pub fn get_mut (& 's mut self , key : CowStr < 'input >) -> Option < & 's mut FootnoteDef > { self . 0 . get_mut (& UniCase :: new (key)) } }
};
}
