// Generated macro for Drain (struct)
macro_rules! Depcrate_stringDrain {
() => {
// Module: crate::string
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " Draining parallel iterator that moves a range of characters out of a string,"] # [doc = " but keeps the total capacity."] # [derive (Debug)] pub struct Drain < 'a > { string : & 'a mut String , range : Range < usize > , }
};
}
